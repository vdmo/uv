//! The server: message loop, state, the analysis worker, and publishing diagnostics.
//! The requests it answers are in `handlers`.

use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use serde_json::{json, Value};
use uv_project::target_profile::TargetProfile;
use uv_tooling::analysis::{analyze_workspace, AnalysisSnapshot, ToolingAnalysisOptions};
use uv_tooling::document_store::{DocumentOverlay, DocumentStore};
use uv_tooling::uri::{file_uri_to_path, normalize_path, path_key, path_to_file_uri};

use crate::completion::apply_content_changes;
use crate::config::*;
use crate::diagnostics::{diagnostics_for_path, DiagnosticTextCache};
use crate::jsonx::{get_i64, get_str, text_document_uri};
use crate::modules::*;

const ANALYSIS_DEBOUNCE: Duration = Duration::from_millis(100);

#[derive(Default, Clone)]
pub struct LspServerOptions {
    pub log_file: Option<PathBuf>,
    pub target_profile: Option<TargetProfile>,
}

pub struct ProjectSnapshot {
    pub snapshot: AnalysisSnapshot,
}

struct PendingAnalysis {
    root_key: String,
    generation: u64,
    options: ToolingAnalysisOptions,
    overlays: Vec<DocumentOverlay>,
}

#[derive(Default)]
struct AnalysisQueue {
    pending: VecDeque<PendingAnalysis>,
    generations: HashMap<String, u64>,
    stop: bool,
}

#[derive(Default, Clone, Copy)]
pub struct ClientFlags {
    pub declaration_link_support: bool,
    pub definition_link_support: bool,
    pub implementation_link_support: bool,
    pub type_definition_link_support: bool,
    pub hierarchical_document_symbol_support: bool,
    pub workspace_edit_document_changes_support: bool,
    pub completion_snippet_support: bool,
    pub watched_files_dynamic_registration: bool,
    pub workspace_configuration_supported: bool,
    pub semantic_token_refresh_supported: bool,
    pub inlay_hint_refresh_supported: bool,
    pub code_lens_refresh_supported: bool,
    pub diagnostic_refresh_supported: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TraceMode {
    Off,
    Messages,
    Verbose,
}

/// What the main thread and the analysis worker share.
pub struct Core {
    log: Mutex<Option<std::fs::File>>,
    output: Mutex<()>,
    projects_by_root: Mutex<HashMap<String, Arc<ProjectSnapshot>>>,
    analysis: Mutex<AnalysisQueue>,
    analysis_cv: Condvar,
    flags: Mutex<ClientFlags>,
    next_server_request_id: AtomicU64,
}

pub struct Server {
    pub core: Arc<Core>,
    pub options: LspServerOptions,
    pub documents: DocumentStore,
    pub workspace_roots: Vec<PathBuf>,
    pub semantic_token_data_by_result_id: HashMap<String, Vec<i64>>,
    pub semantic_token_result_id_order: VecDeque<String>,
    trace_mode: TraceMode,
    pending_workspace_configuration_request_id: Option<String>,
    initialize_completed: bool,
    shutdown_requested: bool,
    running: bool,
    worker: Option<std::thread::JoinHandle<()>>,
}

fn write_message(out: &mut impl Write, message: &Value) {
    let body = message.to_string();
    let _ = write!(out, "Content-Length: {}\r\n\r\n{}", body.len(), body);
    let _ = out.flush();
}

enum ReadResult {
    Message(Value),
    EndOfStream,
    TooLarge,
    ParseError(String),
}

fn read_message(input: &mut impl BufRead) -> ReadResult {
    const MAX_CONTENT_LENGTH: usize = 64 * 1024 * 1024;
    let mut content_length: Option<usize> = None;
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) | Err(_) => return ReadResult::EndOfStream,
            Ok(_) => {}
        }
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("content-length") {
                if let Ok(length) = value.trim().parse::<usize>() {
                    content_length = Some(length);
                }
            }
        }
    }
    let Some(length) = content_length.filter(|length| *length > 0) else {
        return ReadResult::EndOfStream;
    };
    if length > MAX_CONTENT_LENGTH {
        return ReadResult::TooLarge;
    }
    let mut body = vec![0u8; length];
    if input.read_exact(&mut body).is_err() {
        return ReadResult::EndOfStream;
    }
    match serde_json::from_slice(&body) {
        Ok(value) => ReadResult::Message(value),
        Err(err) => ReadResult::ParseError(err.to_string()),
    }
}

fn jsonrpc_base() -> serde_json::Map<String, Value> {
    let mut map = serde_json::Map::new();
    map.insert("jsonrpc".to_string(), json!("2.0"));
    map
}

fn is_request_id(id: &Value) -> bool {
    id.is_string() || id.is_i64() || id.is_u64()
}

impl Core {
    pub fn log(&self, message: &str) {
        let _guard = self.output.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(file) = self.log.lock().unwrap_or_else(|p| p.into_inner()).as_mut() {
            let _ = writeln!(file, "{message}");
            let _ = file.flush();
        }
    }

    fn send(&self, message: Value) {
        let _guard = self.output.lock().unwrap_or_else(|p| p.into_inner());
        write_message(&mut std::io::stdout().lock(), &message);
    }

    pub fn send_response(&self, id: &Value, result: Value) {
        let mut response = jsonrpc_base();
        response.insert("id".into(), id.clone());
        response.insert("result".into(), result);
        self.send(Value::Object(response));
    }

    pub fn send_error(&self, id: &Value, code: i64, message: &str) {
        let mut response = jsonrpc_base();
        response.insert("id".into(), id.clone());
        response.insert("error".into(), json!({ "code": code, "message": message }));
        self.send(Value::Object(response));
    }

    pub fn send_request(&self, method: &str, params: Value) -> String {
        let id = format!("uv-lsp-{}", self.next_server_request_id.fetch_add(1, Ordering::SeqCst));
        let mut request = jsonrpc_base();
        request.insert("id".into(), json!(id));
        request.insert("method".into(), json!(method));
        request.insert("params".into(), params);
        self.send(Value::Object(request));
        id
    }

    pub fn send_notification(&self, method: &str, params: Value) {
        let mut notification = jsonrpc_base();
        notification.insert("method".into(), json!(method));
        notification.insert("params".into(), params);
        self.send(Value::Object(notification));
    }

    pub fn flags(&self) -> ClientFlags {
        *self.flags.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn request_client_refreshes(&self) {
        let flags = self.flags();
        for (supported, method) in [
            (flags.semantic_token_refresh_supported, "workspace/semanticTokens/refresh"),
            (flags.inlay_hint_refresh_supported, "workspace/inlayHint/refresh"),
            (flags.code_lens_refresh_supported, "workspace/codeLens/refresh"),
            (flags.diagnostic_refresh_supported, "workspace/diagnostic/refresh"),
        ] {
            if supported {
                self.send_request(method, Value::Null);
            }
        }
    }

    pub fn project_for_root_key(&self, key: &str) -> Option<Arc<ProjectSnapshot>> {
        self.projects_by_root.lock().unwrap_or_else(|p| p.into_inner()).get(key).cloned()
    }

    pub fn project_snapshots(&self) -> Vec<Arc<ProjectSnapshot>> {
        self.projects_by_root.lock().unwrap_or_else(|p| p.into_inner()).values().cloned().collect()
    }

    fn publish_diagnostics_for_overlays(&self, overlays: &[DocumentOverlay], snapshot: &AnalysisSnapshot) {
        let mut documents = DocumentStore::default();
        for overlay in overlays {
            documents.open(&overlay.uri, &overlay.path, overlay.version, overlay.text_utf8.clone());
        }
        for overlay in overlays {
            let mut cache = DiagnosticTextCache::new();
            let diagnostics = diagnostics_for_path(snapshot, &documents, &overlay.path, &mut cache);
            self.send_notification("textDocument/publishDiagnostics", json!({ "uri": overlay.uri, "version": overlay.version, "diagnostics": diagnostics }));
        }
    }

    fn analysis_worker_loop(self: Arc<Self>) {
        loop {
            let pending = {
                let mut queue = self.analysis.lock().unwrap_or_else(|p| p.into_inner());
                while !queue.stop && queue.pending.is_empty() {
                    queue = self.analysis_cv.wait(queue).unwrap_or_else(|p| p.into_inner());
                }
                if queue.stop && queue.pending.is_empty() {
                    return;
                }
                let pending = queue.pending.pop_front().expect("non-empty");
                if queue.generations.get(&pending.root_key).is_some_and(|current| pending.generation < *current) {
                    continue;
                }
                // Wait out a burst of edits; a newer request for the root supersedes this one.
                let (guard, _) = self
                    .analysis_cv
                    .wait_timeout_while(queue, ANALYSIS_DEBOUNCE, |queue| !queue.stop && queue.generations.get(&pending.root_key).is_none_or(|latest| *latest <= pending.generation))
                    .unwrap_or_else(|p| p.into_inner());
                queue = guard;
                if queue.stop {
                    return;
                }
                if queue.generations.get(&pending.root_key).is_some_and(|latest| pending.generation < *latest) {
                    continue;
                }
                pending
            };
            let snapshot = analyze_workspace(&pending.options, &pending.overlays);
            let project = Arc::new(ProjectSnapshot { snapshot });
            let install = {
                let queue = self.analysis.lock().unwrap_or_else(|p| p.into_inner());
                queue.generations.get(&pending.root_key) == Some(&pending.generation)
            };
            if !install {
                continue;
            }
            let replaced_existing = {
                let mut projects = self.projects_by_root.lock().unwrap_or_else(|p| p.into_inner());
                projects.insert(pending.root_key.clone(), project.clone()).is_some()
            };
            self.log(&format!("analyzed {}", pending.options.project_root));
            self.publish_diagnostics_for_overlays(&pending.overlays, &project.snapshot);
            if replaced_existing {
                self.request_client_refreshes();
            }
        }
    }
}

impl Server {
    pub fn new(options: LspServerOptions) -> Server {
        let log = options.log_file.as_ref().and_then(|path| std::fs::OpenOptions::new().create(true).append(true).open(path).ok());
        let core = Arc::new(Core {
            log: Mutex::new(log),
            output: Mutex::new(()),
            projects_by_root: Mutex::new(HashMap::new()),
            analysis: Mutex::new(AnalysisQueue::default()),
            analysis_cv: Condvar::new(),
            flags: Mutex::new(ClientFlags::default()),
            next_server_request_id: AtomicU64::new(1),
        });
        let worker_core = core.clone();
        let worker = std::thread::Builder::new().stack_size(1 << 30).spawn(move || worker_core.analysis_worker_loop()).ok();
        Server {
            core,
            options,
            documents: DocumentStore::default(),
            workspace_roots: Vec::new(),
            semantic_token_data_by_result_id: HashMap::new(),
            semantic_token_result_id_order: VecDeque::new(),
            trace_mode: TraceMode::Off,
            pending_workspace_configuration_request_id: None,
            initialize_completed: false,
            shutdown_requested: false,
            running: true,
            worker,
        }
    }

    pub fn log(&self, message: &str) {
        self.core.log(message);
    }

    pub fn flags(&self) -> ClientFlags {
        self.core.flags()
    }

    pub fn set_flags(&self, flags: ClientFlags) {
        *self.core.flags.lock().unwrap_or_else(|p| p.into_inner()) = flags;
    }

    fn stop_analysis_worker(&mut self) {
        {
            let mut queue = self.core.analysis.lock().unwrap_or_else(|p| p.into_inner());
            queue.stop = true;
            queue.pending.clear();
        }
        self.core.analysis_cv.notify_all();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    pub fn run(&mut self) -> i32 {
        self.log("server start");
        let stdin = std::io::stdin();
        let mut input = stdin.lock();
        while self.running {
            match read_message(&mut input) {
                ReadResult::EndOfStream => break,
                ReadResult::TooLarge => {
                    self.log("JSON-RPC message too large");
                    self.core.send_error(&Value::Null, -32700, "Message too large");
                    break;
                }
                ReadResult::ParseError(err) => {
                    self.log(&format!("JSON parse error: {err}"));
                    self.core.send_error(&Value::Null, -32700, "Parse error");
                }
                ReadResult::Message(message) => {
                    if !message.is_object() {
                        self.core.send_error(&Value::Null, -32600, "Invalid Request");
                        continue;
                    }
                    self.handle_message(&message);
                }
            }
        }
        self.stop_analysis_worker();
        self.log("server stop");
        if self.shutdown_requested {
            0
        } else {
            1
        }
    }

    fn handle_message(&mut self, message: &Value) {
        let method = message.get("method").and_then(Value::as_str);
        let id = message.get("id");
        let has_response_payload = message.get("result").is_some() || message.get("error").is_some();
        if message.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            if let (Some(id), false) = (id, has_response_payload) {
                self.core.send_error(if is_request_id(id) { id } else { &Value::Null }, -32600, "Invalid Request");
            }
            return;
        }
        let Some(method) = method else {
            if let Some(id) = id {
                if has_response_payload {
                    if is_request_id(id) {
                        self.handle_response(message, id);
                    }
                } else {
                    self.core.send_error(if is_request_id(id) { id } else { &Value::Null }, -32600, "Invalid Request");
                }
            }
            return;
        };
        match id {
            Some(id) => {
                if !is_request_id(id) {
                    self.core.send_error(&Value::Null, -32600, "Invalid Request");
                    return;
                }
                self.handle_request(message, id, method);
            }
            None => self.handle_notification(message, method),
        }
    }

    fn handle_request(&mut self, message: &Value, id: &Value, method: &str) {
        let params = message.get("params").filter(|params| params.is_object());
        if !self.initialize_completed && method != "initialize" {
            self.core.send_error(id, -32002, "Server is not initialized");
            return;
        }
        if self.shutdown_requested {
            self.core.send_error(id, -32600, "Server has shut down");
            return;
        }
        if method == "initialize" {
            if self.initialize_completed {
                self.core.send_error(id, -32600, "Server is already initialized");
                return;
            }
            if !client_allows_utf16_position_encoding(params) {
                self.core.send_error(id, -32602, "Client does not support utf-16 positions");
                return;
            }
            let result = self.handle_initialize(params);
            self.core.send_response(id, result);
            self.initialize_completed = true;
            return;
        }
        if method == "shutdown" {
            self.shutdown_requested = true;
            self.core.send_response(id, Value::Null);
            return;
        }
        match self.dispatch_request(method, params) {
            Some(result) => self.core.send_response(id, result),
            None => self.core.send_error(id, -32601, "Method not found"),
        }
    }

    fn handle_response(&mut self, message: &Value, id: &Value) {
        let Some(id) = id.as_str() else {
            return;
        };
        if self.pending_workspace_configuration_request_id.as_deref() != Some(id) {
            return;
        }
        self.pending_workspace_configuration_request_id = None;
        if message.get("error").is_some() {
            self.log("workspace/configuration request failed");
            return;
        }
        let setting = target_profile_setting_from_configuration_response(message);
        if self.apply_target_profile_setting(&setting) {
            self.analyze_and_publish();
        }
    }

    fn handle_notification(&mut self, message: &Value, method: &str) {
        let params = message.get("params").filter(|params| params.is_object());
        if method == "exit" {
            self.running = false;
            return;
        }
        if !self.initialize_completed || self.shutdown_requested {
            return;
        }
        match method {
            "initialized" => {
                self.register_watched_file_notifications();
                self.request_workspace_configuration();
                self.analyze_and_publish();
            }
            "textDocument/didOpen" => self.did_open(params),
            "textDocument/didChange" => self.did_change(params),
            "textDocument/didSave" => self.did_save(params),
            "textDocument/didClose" => self.did_close(params),
            "workspace/didChangeConfiguration" => self.did_change_configuration(params),
            "workspace/didChangeWatchedFiles" | "workspace/didCreateFiles" | "workspace/didRenameFiles" | "workspace/didDeleteFiles" => self.analyze_and_publish(),
            "workspace/didChangeWorkspaceFolders" => self.did_change_workspace_folders(params),
            "$/setTrace" => self.did_set_trace(params),
            _ => {}
        }
    }

    // ---- paths and snapshots ----

    pub fn text_for_path(&self, path: &Path) -> String {
        crate::diagnostics::text_for_path(&self.documents, path).unwrap_or_default()
    }

    pub fn path_from_text_document(&self, params: Option<&Value>) -> Option<PathBuf> {
        file_uri_to_path(&text_document_uri(params)?)
    }

    pub fn manifest_root_for_path(&self, path: &Path) -> Option<PathBuf> {
        let normalized = normalize_path(path);
        let mut current = normalized.clone();
        if !current.is_dir() {
            current = current.parent().map(Path::to_path_buf).unwrap_or_default();
        }
        while !current.as_os_str().is_empty() {
            if current.join("Ultraviolet.toml").exists() {
                return Some(normalize_path(&current));
            }
            match current.parent() {
                Some(parent) if parent != current => current = parent.to_path_buf(),
                _ => break,
            }
        }
        self.workspace_roots.iter().find(|root| path_is_within_root(&normalized, root) && root.join("Ultraviolet.toml").exists()).map(|root| normalize_path(root))
    }

    fn overlays_for_root(&self, root: &Path) -> Vec<DocumentOverlay> {
        let root_key = path_key(root);
        self.documents.overlays().into_iter().filter(|overlay| self.manifest_root_for_path(&overlay.path).is_some_and(|found| path_key(&found) == root_key)).collect()
    }

    pub fn snapshot_for_path(&self, path: &Path) -> Option<Arc<ProjectSnapshot>> {
        let root = self.manifest_root_for_path(path)?;
        self.core.project_for_root_key(&path_key(&root))
    }

    // ---- documents ----

    fn did_open(&mut self, params: Option<&Value>) {
        let Some(text_document) = params.and_then(|params| params.get("textDocument")) else {
            return;
        };
        let (Some(uri), Some(text)) = (get_str(text_document, "uri"), get_str(text_document, "text")) else {
            return;
        };
        let Some(path) = file_uri_to_path(uri) else {
            return;
        };
        let version = get_i64(text_document, "version").unwrap_or(0);
        if let Some(existing) = self.documents.find_by_uri(uri) {
            if version <= existing.version {
                self.log(&format!("ignored non-incrementing textDocument/didOpen for {uri}"));
                return;
            }
            self.documents.change_full(uri, version, text.to_string());
        } else {
            self.documents.open(uri, &path, version, text.to_string());
        }
        self.analyze_path_or_publish_no_manifest(uri, &path);
    }

    fn did_change(&mut self, params: Option<&Value>) {
        let Some(uri) = text_document_uri(params) else {
            return;
        };
        let Some(version) = params.and_then(|params| params.get("textDocument")).and_then(|doc| get_i64(doc, "version")) else {
            return;
        };
        let Some(changes) = params.and_then(|params| params.get("contentChanges")).and_then(Value::as_array).filter(|changes| !changes.is_empty()) else {
            return;
        };
        let Some(existing) = self.documents.find_by_uri(&uri).cloned() else {
            return;
        };
        if version <= existing.version {
            self.log(&format!("ignored non-incrementing textDocument/didChange for {uri}"));
            return;
        }
        let Some(text) = apply_content_changes(existing.text_utf8.clone(), changes) else {
            return;
        };
        self.documents.change_full(&uri, version, text);
        if let Some(path) = file_uri_to_path(&uri) {
            self.analyze_path_or_publish_no_manifest(&uri, &path);
        }
    }

    fn did_save(&mut self, params: Option<&Value>) {
        if let (Some(uri), Some(text)) = (text_document_uri(params), params.and_then(|params| get_str(params, "text"))) {
            if let Some(existing) = self.documents.find_by_uri(&uri) {
                let version = existing.version;
                self.documents.change_full(&uri, version, text.to_string());
            }
        }
        match text_document_uri(params) {
            Some(uri) => {
                if let Some(path) = file_uri_to_path(&uri) {
                    self.analyze_path_or_publish_no_manifest(&uri, &path);
                }
            }
            None => self.analyze_and_publish(),
        }
    }

    fn did_close(&mut self, params: Option<&Value>) {
        let Some(uri) = text_document_uri(params) else {
            return;
        };
        let Some(overlay) = self.documents.find_by_uri(&uri).cloned() else {
            return;
        };
        self.documents.close(&uri);
        self.core.send_notification("textDocument/publishDiagnostics", json!({ "uri": uri, "version": overlay.version, "diagnostics": [] }));
        self.analyze_and_publish();
    }

    fn did_change_configuration(&mut self, params: Option<&Value>) {
        if self.flags().workspace_configuration_supported {
            self.request_workspace_configuration();
            return;
        }
        let setting = target_profile_setting_from_configuration(params);
        if self.apply_target_profile_setting(&setting) {
            self.analyze_and_publish();
        }
    }

    fn apply_target_profile_setting(&mut self, setting: &TargetProfileSetting) -> bool {
        if !setting.present {
            return false;
        }
        if !setting.valid {
            self.log(&format!("ignored invalid targetProfile setting: {}", setting.raw_value));
            return false;
        }
        if self.options.target_profile == setting.value {
            return false;
        }
        self.options.target_profile = setting.value;
        true
    }

    fn drop_project(&mut self, project_key: &str) -> bool {
        let removed = self.core.projects_by_root.lock().unwrap_or_else(|p| p.into_inner()).remove(project_key).is_some();
        let mut queue = self.core.analysis.lock().unwrap_or_else(|p| p.into_inner());
        *queue.generations.entry(project_key.to_string()).or_insert(0) += 1;
        queue.pending.retain(|pending| pending.root_key != project_key);
        removed
    }

    fn did_change_workspace_folders(&mut self, params: Option<&Value>) {
        let Some(event) = params.and_then(|params| params.get("event")) else {
            return;
        };
        let folders = |name: &str| -> Vec<PathBuf> {
            event.get(name).and_then(Value::as_array).into_iter().flatten().filter_map(|folder| get_str(folder, "uri")).filter_map(file_uri_to_path).collect()
        };
        let mut removed_project = false;
        for path in folders("removed") {
            let key = path_key(&path);
            let manifest_root = self.manifest_root_for_path(&path);
            self.workspace_roots.retain(|root| path_key(root) != key);
            let project_key = manifest_root.map(|root| path_key(&root)).unwrap_or(key);
            removed_project = self.drop_project(&project_key) || removed_project;
        }
        let mut added_manifest_roots = Vec::new();
        for path in folders("added") {
            let root = normalize_path(&path);
            let key = path_key(&root);
            if !self.workspace_roots.iter().any(|item| path_key(item) == key) {
                self.workspace_roots.push(root.clone());
            }
            if let Some(manifest_root) = self.manifest_root_for_path(&root) {
                added_manifest_roots.push(manifest_root);
            }
        }
        if removed_project {
            self.analyze_and_publish();
            self.core.request_client_refreshes();
            return;
        }
        for manifest_root in added_manifest_roots {
            self.schedule_project_analysis(&manifest_root);
        }
    }

    fn did_set_trace(&mut self, params: Option<&Value>) {
        match params.and_then(|params| get_str(params, "value")) {
            None | Some("off") => self.trace_mode = TraceMode::Off,
            Some("messages") => {
                self.trace_mode = TraceMode::Messages;
                self.send_trace_log("trace enabled", "");
            }
            Some("verbose") => {
                self.trace_mode = TraceMode::Verbose;
                self.send_trace_log("trace enabled", "verbose");
            }
            Some(_) => {}
        }
    }

    pub fn send_trace_log(&self, message: &str, verbose: &str) {
        if self.trace_mode == TraceMode::Off {
            return;
        }
        let mut params = json!({ "message": message });
        if self.trace_mode == TraceMode::Verbose && !verbose.is_empty() {
            params["verbose"] = json!(verbose);
        }
        self.core.send_notification("$/logTrace", params);
    }

    fn register_watched_file_notifications(&self) {
        if !self.flags().watched_files_dynamic_registration {
            return;
        }
        self.core.send_request(
            "client/registerCapability",
            json!({ "registrations": [{ "id": "ultraviolet-source-watchers", "method": "workspace/didChangeWatchedFiles", "registerOptions": watched_file_registration_options() }] }),
        );
    }

    fn request_workspace_configuration(&mut self) {
        if !self.flags().workspace_configuration_supported {
            return;
        }
        self.pending_workspace_configuration_request_id = Some(self.core.send_request("workspace/configuration", json!({ "items": [{ "section": "ultraviolet.languageServer" }] })));
    }

    // ---- analysis ----

    fn analyze_and_publish(&mut self) {
        let mut roots: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for workspace_root in &self.workspace_roots {
            if let Some(root) = self.manifest_root_for_path(workspace_root) {
                roots.insert(path_key(&root));
            }
        }
        for overlay in self.documents.overlays() {
            if let Some(root) = self.manifest_root_for_path(&overlay.path) {
                roots.insert(path_key(&root));
            }
        }
        let removed: HashSet<String> = {
            let mut projects = self.core.projects_by_root.lock().unwrap_or_else(|p| p.into_inner());
            let gone: Vec<String> = projects.keys().filter(|key| !roots.contains(*key)).cloned().collect();
            for key in &gone {
                projects.remove(key);
            }
            gone.into_iter().collect()
        };
        if !removed.is_empty() {
            {
                let mut queue = self.core.analysis.lock().unwrap_or_else(|p| p.into_inner());
                for key in &removed {
                    *queue.generations.entry(key.clone()).or_insert(0) += 1;
                }
                queue.pending.retain(|pending| !removed.contains(&pending.root_key));
            }
            self.core.request_client_refreshes();
        }
        for root_key in &roots {
            let from_workspace = self.workspace_roots.iter().filter_map(|workspace_root| self.manifest_root_for_path(workspace_root)).find(|root| path_key(root) == *root_key);
            let from_overlay = || self.documents.overlays().into_iter().filter_map(|overlay| self.manifest_root_for_path(&overlay.path)).find(|root| path_key(root) == *root_key);
            if let Some(root) = from_workspace.or_else(from_overlay) {
                self.schedule_project_analysis(&root);
            }
        }
        self.publish_diagnostics_for_open_documents();
    }

    fn schedule_project_analysis(&self, project_root: &Path) {
        let root = normalize_path(project_root);
        let options = ToolingAnalysisOptions {
            project_root: root.to_string_lossy().into_owned(),
            target_profile: self.options.target_profile,
            fallback_target_profile: Some(host_default_target_profile()),
            run_comptime: true,
            semantic: true,
            ..Default::default()
        };
        let overlays = self.overlays_for_root(&root);
        let root_key = path_key(&root);
        {
            let mut queue = self.core.analysis.lock().unwrap_or_else(|p| p.into_inner());
            let generation = {
                let entry = queue.generations.entry(root_key.clone()).or_insert(0);
                *entry += 1;
                *entry
            };
            queue.pending.retain(|pending| pending.root_key != root_key);
            queue.pending.push_back(PendingAnalysis { root_key, generation, options, overlays });
        }
        self.core.analysis_cv.notify_one();
    }

    fn analyze_path_or_publish_no_manifest(&self, uri: &str, path: &Path) {
        match self.manifest_root_for_path(path) {
            Some(root) => self.schedule_project_analysis(&root),
            None => self.publish_no_manifest_diagnostic(uri, path),
        }
    }

    fn publish_diagnostics_for_open_documents(&self) {
        for overlay in self.documents.overlays() {
            self.publish_diagnostics_for_uri(&overlay.uri);
        }
    }

    fn attach_version(&self, params: &mut Value, uri: &str) {
        if let Some(overlay) = self.documents.find_by_uri(uri) {
            params["version"] = json!(overlay.version);
        }
    }

    fn publish_diagnostics_for_uri(&self, uri: &str) {
        let Some(path) = file_uri_to_path(uri) else {
            return;
        };
        let mut params = json!({ "uri": uri });
        match self.snapshot_for_path(&path) {
            None => {
                if self.manifest_root_for_path(&path).is_some() {
                    self.attach_version(&mut params, uri);
                    params["diagnostics"] = json!([]);
                    self.core.send_notification("textDocument/publishDiagnostics", params);
                } else {
                    self.publish_no_manifest_diagnostic(uri, &path);
                }
            }
            Some(project) => {
                self.attach_version(&mut params, uri);
                params["diagnostics"] = Value::Array(diagnostics_for_path(&project.snapshot, &self.documents, &path, &mut DiagnosticTextCache::new()));
                self.core.send_notification("textDocument/publishDiagnostics", params);
            }
        }
    }

    fn publish_no_manifest_diagnostic(&self, uri: &str, path: &Path) {
        let mut params = json!({ "uri": uri });
        self.attach_version(&mut params, uri);
        params["diagnostics"] = json!([diagnostic_at_start(&format!("No Ultraviolet.toml manifest found for {}", normalize_path(path).to_string_lossy()))]);
        self.core.send_notification("textDocument/publishDiagnostics", params);
    }

    pub fn diagnostic_report_for_path(&self, path: &Path, previous_result_id: Option<&str>, cache: &mut DiagnosticTextCache) -> Value {
        let mut diagnostics: Vec<Value> = Vec::new();
        match self.snapshot_for_path(path) {
            Some(project) => diagnostics = diagnostics_for_path(&project.snapshot, &self.documents, path, cache),
            None => {
                if self.manifest_root_for_path(path).is_none() {
                    diagnostics.push(diagnostic_at_start(&format!("No Ultraviolet.toml manifest found for {}", normalize_path(path).to_string_lossy())));
                }
            }
        }
        let result_id = diagnostic_result_id_for_path(path, &diagnostics);
        if previous_result_id == Some(result_id.as_str()) {
            return unchanged_diagnostic_report(&result_id);
        }
        full_diagnostic_report(diagnostics, Some(result_id))
    }

    pub fn workspace_diagnostic_report_for_path(&self, path: &Path, previous: &HashMap<String, String>, cache: &mut DiagnosticTextCache) -> Value {
        let uri = path_to_file_uri(path);
        let mut report = self.diagnostic_report_for_path(path, previous.get(&uri).map(String::as_str), cache);
        report["uri"] = json!(uri);
        if let Some(overlay) = self.documents.find_by_path(path) {
            report["version"] = json!(overlay.version);
        }
        report
    }

    pub fn reference_locations_for_symbol(&self, symbol: &uv_analysis::language_service::LanguageSymbolInfo, include_declaration: bool) -> Vec<Value> {
        let Some(project) = self.snapshot_for_path(Path::new(&*symbol.selection_range.file)) else {
            return Vec::new();
        };
        let mut index_by_file: HashMap<String, uv_tooling::line_index::LineIndex> = HashMap::new();
        let mut locations = Vec::new();
        for reference in project.snapshot.language_service.references_for_symbol(&symbol.id, include_declaration) {
            let file = Path::new(&*reference.range.file);
            let key = path_key(file);
            let index = index_by_file.entry(key).or_insert_with(|| uv_tooling::line_index::LineIndex::new(&self.text_for_path(file)));
            locations.push(json!({ "uri": path_to_file_uri(file), "range": crate::jsonx::range_json(index.range_for(&reference.range)) }));
        }
        locations
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop_analysis_worker();
    }
}
