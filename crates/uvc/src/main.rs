//! `uvc`, the Ultraviolet compiler driver.
//!
//! This build implements the driver through phase 1 (project loading, lexing, parsing and
//! the syntactic checks). Compile-time execution and every later phase are not ported yet:
//! `--phase1-only` runs to completion, and any other run that gets past phase 1 without
//! errors says so and exits with status 3 instead of reporting a successful build.

mod cli;
mod sema;

use std::collections::HashMap;
use std::time::Instant;

use cli::{CliOptions, ColorMode};
use uv_analysis::ffi::unwind_surface::collect_ffi_surface;
use uv_core::behavior_model::ErrorRecoveryPolicy;
use uv_core::diagnostic_messages::{emit_external_diagnostic, make_diagnostic_by_id};
use uv_core::diagnostic_render::{diagnostic_summary, order, render, render_rich, RenderOptions};
use uv_core::diagnostics::{
    compile_status, emit, has_error, CompileStatusResult, Diagnostic, DiagnosticStream, Severity,
};
use uv_core::host::{current_host_process_id, HostStream};
use uv_core::process_config::{
    build_progress_override, get_verbosity, is_debug_enabled, manifest_build_progress,
    max_errors_override, set_build_progress_override, set_debug_subsystems,
    set_incremental_override, set_link_debug_override, set_max_errors_override,
    set_out_dir_override, set_runtime_lib_override, set_verbosity, Verbosity,
};
use uv_core::spec_rule;
use uv_core::spec_trace::Conformance;
use uv_core::terminal::{
    colorize, is_color_enabled_with_override, terminal_width, Color, ColorOverride,
};
use uv_project::load_project::{load_project, parse_assembly_target};
use uv_project::manifest::find_project_root;
use uv_project::outputs::dump_project;
use uv_project::project::{Assembly, Project};
use uv_project::target_profile::{target_profile_name, TargetProfile};
use uv_core::symbols::string_of_path;
use uv_source::ast::item_summary;
use uv_source::phase1::{emit_internal_diagnostic, run_phase1, AssemblyOutcome, Phase1Observer};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Exit status for a run that reached a compiler phase this build does not implement.
const EXIT_PHASE_NOT_IMPLEMENTED: i32 = 3;

fn print_version() {
    println!("Ultraviolet {VERSION}");
    println!("Ultraviolet Language Compiler");
    println!("Spec: Ultraviolet Language Specification");
}

fn resolve_command_name(argv0: Option<&String>) -> String {
    argv0
        .and_then(|arg| std::path::Path::new(arg).file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "uvc".to_string())
}

fn build_progress_enabled() -> bool {
    build_progress_override().or_else(manifest_build_progress).unwrap_or(true)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BuildLogMode {
    None,
    Summary,
    Detailed,
}

fn resolve_phase_log_mode(channel_enabled: bool, debug_enabled: bool) -> BuildLogMode {
    if !channel_enabled {
        BuildLogMode::None
    } else if debug_enabled || get_verbosity() == Verbosity::Verbose {
        BuildLogMode::Detailed
    } else {
        BuildLogMode::Summary
    }
}

fn should_emit_summary_phase_message(message: &str) -> bool {
    const PREFIXES: [&str; 13] = [
        "event=start",
        "event=finish",
        "phase=project-load",
        "phase=parse-modules",
        "phase=incremental-fastpath",
        "phase=sema step=name-map-collect-finish",
        "phase=sema step=resolve-finish",
        "phase=sema step=typecheck-finish",
        "phase=codegen cache=build-start",
        "phase=codegen cache=build-finish",
        "phase=codegen incremental=fingerprint-start",
        "phase=codegen incremental=fingerprint-finish",
        "phase=codegen incremental=disabled",
    ];
    message == "phase=sema"
        || message == "phase=codegen"
        || PREFIXES.iter().any(|prefix| message.starts_with(prefix))
}

struct BuildLog {
    show_progress: bool,
    use_color: bool,
    debug_pipeline: bool,
    mode: BuildLogMode,
}

impl BuildLog {
    /// Human-friendly progress: right-aligned colored label followed by detail.
    fn progress(&self, label: &str, detail: &str, color: Color) {
        if !self.show_progress {
            return;
        }
        let pad = 12usize.saturating_sub(label.len());
        eprintln!("{}{}  {detail}", " ".repeat(pad), colorize(label, color, self.use_color));
    }

    /// Machine-format phase log.
    fn machine(&self, message: &str) {
        if self.debug_pipeline {
            eprintln!("[trace][build] pid={} {message}", current_host_process_id());
            return;
        }
        match self.mode {
            BuildLogMode::None => {}
            BuildLogMode::Summary if !should_emit_summary_phase_message(message) => {}
            _ => eprintln!("[info][build] {message}"),
        }
    }
}

fn resolve_selected_target_profile(
    opts: &CliOptions,
    project: &Project,
    diags: &mut DiagnosticStream,
) -> Option<TargetProfile> {
    let record = |source: &str, selected: TargetProfile| {
        if Conformance::enabled() {
            let profile = target_profile_name(selected);
            Conformance::record_at(
                "req.TargetProfileResolution",
                None,
                &format!("source={source};resolution_once=true;selected={profile}"),
            );
            Conformance::record_at(
                "req.TargetProfileNoHostInference",
                None,
                &format!("host_inference=false;selected={profile}"),
            );
        }
    };
    if let Some(selected) = opts.target_profile_override {
        record("cli_override", selected);
        return Some(selected);
    }
    if let Some(selected) = project.toolchain.target_profile {
        record("toolchain.target_profile", selected);
        return Some(selected);
    }
    if Conformance::enabled() {
        Conformance::record_at(
            "req.TargetProfileNoHostInference",
            None,
            "host_inference=false;selected=none;status=ill_formed",
        );
    }
    emit_external_diagnostic(diags, "E-PRJ-0112");
    None
}

fn count_error_diagnostics(diags: &[Diagnostic]) -> usize {
    diags.iter().filter(|diag| diag.severity == Severity::Error).count()
}

/// Keeps diagnostics up to and including the error that reaches the policy's cap.
fn truncate_diagnostics_to_error_cap(diags: &mut DiagnosticStream, policy: &ErrorRecoveryPolicy) {
    let Some(cap) = policy.max_error_count else {
        return;
    };
    let mut error_count = 0usize;
    let mut kept = 0usize;
    for diag in diags.iter() {
        if diag.severity == Severity::Error {
            if error_count >= cap {
                break;
            }
            error_count += 1;
            kept += 1;
            if error_count >= cap {
                break;
            }
            continue;
        }
        kept += 1;
    }
    diags.truncate(kept);
}

struct LogObserver<'a>(&'a BuildLog);

impl Phase1Observer for LogObserver<'_> {
    fn assembly_start(&self, assembly: &Assembly) {
        self.0.progress(
            "Parsing",
            &format!("{} ({} modules)", assembly.name, assembly.modules.len()),
            Color::BoldGreen,
        );
        self.0.machine(&format!(
            "phase=parse-modules assembly-start name={} modules={} source_root={}",
            assembly.name,
            assembly.modules.len(),
            assembly.source_root
        ));
    }

    fn assembly_finish(&self, assembly: &Assembly, outcome: AssemblyOutcome) {
        let attr_validation = if outcome.attribute_validation_failed { " attr-validation=true" } else { "" };
        self.0.machine(&format!(
            "phase=parse-modules assembly-finish name={} ok={}{attr_validation} parsed_modules={} emitted_diags={}",
            assembly.name, outcome.ok, outcome.parsed_modules, outcome.emitted_diags
        ));
    }
}

fn render_cli_parse_diagnostic(code: &str, message: &str, diag_json: bool) {
    let mut diags = DiagnosticStream::new();
    if let Some(mut diag) = make_diagnostic_by_id(code, None) {
        if !message.is_empty() {
            diag.message = message.to_string();
        }
        diags.push(diag);
    }
    if diag_json {
        println!("{}", cli::diagnostic_stream_to_json(&order(&diags), None));
        return;
    }
    for diag in order(&diags) {
        eprintln!("{} ({}): {}", diag.code, cli::severity_string(diag.severity), diag.message);
    }
}

fn not_implemented(what: &str) -> i32 {
    eprintln!("error: {what} is not implemented in this build of uvc (Rust port in progress)");
    EXIT_PHASE_NOT_IMPLEMENTED
}

fn run_build(opts: &CliOptions, color_override: ColorOverride, error_policy: &ErrorRecoveryPolicy) -> i32 {
    let project_root = find_project_root(&opts.input_path);
    if let Some(conformance_path) = &opts.conformance_path {
        return not_implemented(&format!("--conformance {conformance_path}"));
    }
    let mut diags = DiagnosticStream::new();
    let is_verbose = opts.verbose;
    let show_build_progress = is_verbose || build_progress_enabled();
    let use_color = is_color_enabled_with_override(HostStream::Stderr, color_override);
    let build_start = Instant::now();
    let debug_pipeline = is_debug_enabled("pipeline");
    let log = BuildLog {
        show_progress: show_build_progress,
        use_color,
        debug_pipeline,
        mode: resolve_phase_log_mode(show_build_progress, debug_pipeline),
    };
    log.machine(&format!(
        "event=start input={}{}",
        opts.input_path,
        opts.assembly_target.as_ref().map(|name| format!(" assembly={name}")).unwrap_or_default()
    ));
    log.machine("phase=project-load");
    let Some(assembly_target) = parse_assembly_target(opts.assembly_target.as_deref()) else {
        emit_external_diagnostic(&mut diags, "E-PRJ-0205");
        for diag in &diags {
            eprintln!("{}", render(diag));
        }
        return 1;
    };
    let project_result = load_project(&project_root, &assembly_target);
    for diag in project_result.diags {
        emit(&mut diags, diag);
    }
    let project = project_result.project;
    if !has_error(&diags) && project.is_some() && opts.test_harness_assembly.is_some() {
        return not_implemented("the source-native test harness build");
    }
    let mut selected_target_profile = None;
    if let (false, Some(project)) = (has_error(&diags), &project) {
        selected_target_profile = resolve_selected_target_profile(opts, project, &mut diags);
    }
    let mut sema_pending: Option<String> = None;
    if let (false, Some(project), Some(target_profile)) =
        (has_error(&diags), &project, selected_target_profile)
    {
        let root_name = std::path::Path::new(&project_root)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        log.progress(
            "Loading",
            &format!("{root_name} ({} modules)", project.modules.len()),
            Color::BoldGreen,
        );
        if is_verbose {
            for module in &project.modules {
                eprintln!("       module: {}", module.path);
            }
        }
        log.machine("phase=parse-modules");
        Conformance::set_phase("parse");
        let mut phase1 = run_phase1(project, &LogObserver(&log));
        let mut phase1_ok = phase1.ok;
        let parse_phase_error_count =
            count_error_diagnostics(&diags) + count_error_diagnostics(&phase1.diags);
        if error_policy.max_error_count.is_some_and(|cap| parse_phase_error_count >= cap) {
            spec_rule!("Abort-On-ErrorCount");
            log.machine(&format!(
                "phase=parse-modules abort-on-error-count errors={parse_phase_error_count}"
            ));
            phase1_ok = false;
        }
        let parse_failed_without_error = !phase1_ok && !has_error(&phase1.diags);
        for diag in std::mem::take(&mut phase1.diags) {
            emit(&mut diags, diag);
        }
        if parse_failed_without_error {
            spec_rule!("ResolveModules-Err-Parse");
            emit_internal_diagnostic(
                &mut diags,
                None,
                "Internal error: resolver was asked to continue after parse failure without a parse diagnostic.",
            );
        }
        if phase1_ok {
            let (mut ffi_import_count, mut ffi_export_count) = (0usize, 0usize);
            for module in phase1.reachable_modules() {
                let surface = collect_ffi_surface(module);
                ffi_import_count += surface.imports.len();
                ffi_export_count += surface.exports.len();
            }
            log.machine(&format!(
                "phase=parse-modules step=ffi-surface imports={ffi_import_count} exports={ffi_export_count}"
            ));
        }
        if phase1_ok && opts.dump_ast {
            for module in phase1.reachable_modules() {
                println!("module {}", string_of_path(&module.path));
                for item in &module.items {
                    println!("  {}", item_summary(item, true));
                }
            }
        }
        if phase1_ok && !opts.phase1_only && opts.dump_project {
            for line in dump_project(project, target_profile, true) {
                println!("{line}");
            }
        }
        if phase1_ok && !opts.phase1_only {
            let outcome = sema::run_sema(project, &phase1, target_profile, &mut diags, |verb, what| log.progress(verb, what, Color::BoldGreen));
            sema_pending = outcome.pending;
        }
    }
    let reached_unimplemented_phase = sema_pending.is_some();
    truncate_diagnostics_to_error_cap(&mut diags, error_policy);

    let mut source_cache: HashMap<String, Option<String>> = HashMap::new();
    let mut source_registry = |path: &str| -> Option<String> {
        source_cache
            .entry(path.to_string())
            .or_insert_with(|| {
                std::fs::read(path).ok().map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            })
            .clone()
    };
    let ordered = order(&diags);
    if opts.diag_json {
        println!("{}", cli::diagnostic_stream_to_json(&ordered, Some(&mut source_registry)));
    } else {
        if !ordered.is_empty() && show_build_progress {
            eprintln!();
        }
        let render_opts =
            RenderOptions { color: use_color, terminal_width: terminal_width(), context_lines: 1 };
        for diag in &ordered {
            let lookup = |path: &str| source_registry(path);
            eprintln!("{}", render_rich_with(diag, lookup, &render_opts));
        }
        let summary = diagnostic_summary(&ordered, use_color);
        if !summary.is_empty() {
            eprintln!("\n{summary}");
        }
    }
    if reached_unimplemented_phase {
        return not_implemented(sema_pending.as_deref().unwrap_or("every later compiler phase"));
    }
    let ok = compile_status(&diags) == CompileStatusResult::Ok;
    if !opts.diag_json && show_build_progress {
        let ms = build_start.elapsed().as_millis();
        let elapsed = if ms < 1000 {
            format!("{ms}ms")
        } else {
            format!("{}.{:02}s", ms / 1000, (ms % 1000) / 10)
        };
        if ok {
            log.progress("Finished", &format!("build succeeded in {elapsed}"), Color::BoldGreen);
        } else {
            log.progress("Finished", &format!("build failed in {elapsed}"), Color::BoldRed);
        }
    }
    if ok {
        0
    } else {
        1
    }
}

fn render_rich_with(
    diag: &uv_core::diagnostics::Diagnostic,
    mut lookup: impl FnMut(&str) -> Option<String>,
    opts: &RenderOptions,
) -> String {
    let content = diag.span.as_ref().and_then(|span| lookup(&span.file));
    let sources = |_: &str| content.clone();
    render_rich(diag, Some(&sources), opts)
}

fn run(argv: &[String]) -> i32 {
    let command_name = resolve_command_name(argv.first());
    let opts = match cli::parse_args(argv) {
        Ok(opts) => opts,
        Err(error) => {
            if let Some(code) = error.code {
                let diag_json = argv.iter().skip(1).any(|arg| arg == "--diag-json");
                render_cli_parse_diagnostic(code, &error.message, diag_json);
                return 2;
            }
            if !error.message.is_empty() {
                eprintln!("error: {}", error.message);
            }
            cli::print_usage(&command_name);
            return 2;
        }
    };
    set_build_progress_override(opts.build_progress);
    set_incremental_override(opts.incremental);
    set_runtime_lib_override(opts.runtime_lib_path.clone());
    set_link_debug_override(opts.link_debug);
    set_max_errors_override(opts.max_errors_override);
    set_out_dir_override(opts.out_dir.clone());
    if !opts.debug_subsystems.is_empty() {
        set_debug_subsystems(&opts.debug_subsystems);
    }
    let effective_error_policy: ErrorRecoveryPolicy = max_errors_override().unwrap_or_default();
    set_verbosity(if opts.verbose { Verbosity::Verbose } else { Verbosity::Normal });
    let color_override = match opts.color_mode {
        ColorMode::Always => ColorOverride::ForceOn,
        ColorMode::Never => ColorOverride::ForceOff,
        ColorMode::Auto => ColorOverride::Auto,
    };
    if opts.show_help {
        cli::print_help(&command_name);
        return 0;
    }
    if opts.show_debug_help {
        cli::print_debug_help();
        return 0;
    }
    if opts.show_version {
        print_version();
        return 0;
    }
    if opts.do_test {
        return not_implemented("the `test` command");
    }
    if opts.do_init {
        return not_implemented("the `init` command");
    }
    if opts.do_clean {
        return not_implemented("the `clean` command");
    }
    run_build(&opts, color_override, &effective_error_policy)
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    // The parser and the tree walks recurse with the nesting depth of the source.
    let status = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || run(&argv))
        .expect("failed to start the compiler thread")
        .join()
        .unwrap_or(101);
    Conformance::flush();
    std::process::exit(status);
}
