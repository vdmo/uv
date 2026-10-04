//! Conformance trace recorder (`--conformance`).

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::host::{current_host_process_id, current_host_thread_id};
use crate::path::{normalize, relative};
use crate::span::Span;

#[derive(Default)]
struct TraceState {
    out: Option<BufWriter<File>>,
    domain: String,
    phase: String,
    root: String,
    empty_rule_records: HashSet<String>,
}

static ENABLED: AtomicBool = AtomicBool::new(false);

fn state() -> MutexGuard<'static, TraceState> {
    static STATE: OnceLock<Mutex<TraceState>> = OnceLock::new();
    STATE
        .get_or_init(|| Mutex::new(TraceState::default()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn encode_payload(payload: &str) -> String {
    let mut out = String::with_capacity(payload.len() + 8);
    for c in payload.chars() {
        match c {
            '\t' => out.push_str("%09"),
            '\n' => out.push_str("%0A"),
            '\r' => out.push_str("%0D"),
            '%' => out.push_str("%25"),
            ';' => out.push_str("%3B"),
            '=' => out.push_str("%3D"),
            other => out.push(other),
        }
    }
    out
}

fn timestamp_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

fn rel_path(path: &str, root: &str) -> String {
    let norm = normalize(path);
    let fallback = |norm: String| if norm.is_empty() { "-".to_string() } else { norm };
    if root.is_empty() {
        return fallback(norm);
    }
    match relative(&norm, root) {
        Some(rel) if !rel.is_empty() => rel,
        _ => fallback(norm),
    }
}

fn payload_has_field(payload: &str, key: &str) -> bool {
    payload
        .split(';')
        .any(|segment| segment.split_once('=').is_some_and(|(name, _)| name == key))
}

fn default_category_for_rule(rule_id: &str) -> &'static str {
    if rule_id.starts_with("Log-") {
        "log"
    } else if rule_id.starts_with("Diag-") {
        "diagnostic"
    } else {
        "runtime"
    }
}

fn default_level_for_rule(rule_id: &str) -> &'static str {
    if rule_id.starts_with("Log-") {
        "info"
    } else if rule_id.starts_with("Diag-") {
        "warning"
    } else if rule_id.starts_with("Panic")
        || rule_id.starts_with("RuntimePanic")
        || rule_id.starts_with("PanicCheck")
    {
        "error"
    } else {
        "trace"
    }
}

pub struct Conformance;

impl Conformance {
    pub fn init(path: &str, domain: &str) {
        let mut state = state();
        ENABLED.store(false, Ordering::Relaxed);
        if let Some(mut out) = state.out.take() {
            let _ = out.flush();
        }
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        let Ok(file) = File::create(path) else {
            return;
        };
        let mut out = BufWriter::with_capacity(1 << 20, file);
        let _ = out.write_all(b"compile_conformance_v1\n");
        state.out = Some(out);
        state.domain = domain.to_string();
        state.phase.clear();
        state.empty_rule_records.clear();
        ENABLED.store(true, Ordering::Relaxed);
    }

    pub fn set_root(root: &str) {
        state().root = normalize(root);
    }

    pub fn set_phase(phase: &str) {
        state().phase = phase.to_string();
    }

    pub fn flush() {
        if let Some(out) = state().out.as_mut() {
            let _ = out.flush();
        }
    }

    pub fn record_at(rule_id: &str, span: Option<&Span>, payload: &str) {
        if !ENABLED.load(Ordering::Relaxed) {
            return;
        }
        let mut state = state();
        if !ENABLED.load(Ordering::Relaxed) {
            return;
        }
        let phase = if state.phase.is_empty() { "-".to_string() } else { state.phase.clone() };
        if span.is_none() && payload.is_empty() {
            let record_key = format!("{phase}\t{rule_id}");
            if !state.empty_rule_records.insert(record_key) {
                return;
            }
        }
        let file = span.map_or_else(|| "-".to_string(), |sp| rel_path(&sp.file, &state.root));
        let mut payload_with_meta = String::with_capacity(payload.len() + 128);
        payload_with_meta.push_str(&format!(
            "ts_ms={};pid={};tid={}",
            timestamp_ms(),
            current_host_process_id(),
            current_host_thread_id()
        ));
        if !payload_has_field(payload, "level") {
            payload_with_meta.push_str(";level=");
            payload_with_meta.push_str(default_level_for_rule(rule_id));
        }
        if !payload_has_field(payload, "category") {
            payload_with_meta.push_str(";category=");
            payload_with_meta.push_str(default_category_for_rule(rule_id));
        }
        if !payload.is_empty() {
            payload_with_meta.push(';');
            payload_with_meta.push_str(payload);
        }
        let encoded = encode_payload(&payload_with_meta);
        let position = match span {
            Some(sp) => format!(
                "{}\t{}\t{}\t{}\t",
                sp.start_line, sp.start_col, sp.end_line, sp.end_col
            ),
            None => "-\t-\t-\t-\t".to_string(),
        };
        let line = format!("{}\t{phase}\t{rule_id}\t{file}\t{position}{encoded}\n", state.domain);
        if let Some(out) = state.out.as_mut() {
            let _ = out.write_all(line.as_bytes());
        }
    }

    pub fn record(rule_id: &str) {
        Self::record_at(rule_id, None, "");
    }

    pub fn record_if_enabled(rule_id: &str) {
        if Self::enabled() {
            Self::record(rule_id);
        }
    }

    #[inline]
    pub fn enabled() -> bool {
        ENABLED.load(Ordering::Relaxed)
    }
}
