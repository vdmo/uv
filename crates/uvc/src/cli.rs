//! Command-line parsing and diagnostic JSON rendering.

use uv_core::behavior_model::ErrorRecoveryPolicy;
use uv_core::diagnostics::{Diagnostic, Severity, SubDiagnostic, SubDiagnosticKind};
use uv_core::process_config::{has_debug_subsystems, set_debug_subsystems};
use uv_core::span::Span;
use uv_project::target_profile::{parse_target_profile, TargetProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Debug, Clone, Default)]
pub struct CliOptions {
    pub diag_json: bool,
    pub show_help: bool,
    pub show_debug_help: bool,
    pub show_version: bool,
    pub check_only: bool,
    pub no_crash_report: bool,
    pub phase1_only: bool,
    pub no_output: bool,
    pub dump_project: bool,
    pub dump_ast: bool,
    pub profile_compiler: bool,
    pub conformance_path: Option<String>,
    pub assembly_target: Option<String>,
    pub log_enabled: bool,
    pub log_to_console: bool,
    pub log_to_file: bool,
    pub log_file_path: Option<String>,
    pub trace: bool,
    pub trace_filter_mask: Option<u8>,
    pub trace_min_level: Option<u8>,
    pub build_progress: Option<bool>,
    pub incremental: Option<bool>,
    pub runtime_lib_path: Option<String>,
    pub link_debug: Option<bool>,
    pub max_errors_override: Option<ErrorRecoveryPolicy>,
    pub out_dir: Option<String>,
    pub opt_level: Option<String>,
    pub target_profile_override: Option<TargetProfile>,
    pub debug_subsystems: Vec<String>,
    pub color_mode: ColorMode,
    pub verbose: bool,
    pub input_path: String,
    pub test_target: Option<String>,
    pub test_name_filter: Option<String>,
    pub test_coverage_filter: Option<String>,
    pub test_harness_assembly: Option<String>,
    pub test_harness_module: Option<String>,
    pub test_harness_dir: Option<String>,
    pub emit_ir: bool,
    pub do_init: bool,
    pub do_clean: bool,
    pub do_test: bool,
    pub test_target_rejected: bool,
}

/// A rejected command line: a usage error, or a coded diagnostic when `code` is set.
#[derive(Debug, Clone)]
pub struct CliError {
    pub message: String,
    pub code: Option<&'static str>,
}

const DEBUG_SUBSYSTEMS: [(&str, &str); 22] = [
    ("lex", "Lexer diagnostics and tokenization traces"),
    ("parse", "Parser item-level traces"),
    ("phases", "Frontend phase sequencing"),
    ("pipeline", "Top-level driver pipeline and build coordination"),
    ("output", "Output pipeline, artifacts, and incremental decisions"),
    ("link", "Linker and archiver debugging"),
    ("sema", "Semantic-analysis progress and diagnostics"),
    ("codegen", "IR lowering and code generation traces"),
    ("typeperf", "Semantic-analysis performance counters"),
    ("obj", "Object emission and object-model details"),
    ("binop", "Binary operator lowering/debugging"),
    ("call", "Call resolution and lowering"),
    ("loop", "Loop lowering/debugging"),
    ("parallel", "Parallel lowering/debugging"),
    ("return", "Return-path lowering/debugging"),
    ("union", "Union lowering/debugging"),
    ("wait", "Wait lowering/debugging"),
    ("propagate", "Propagate lowering/debugging"),
    ("spawn", "Spawn lowering/debugging"),
    ("method", "Method resolution/debugging"),
    ("shadow", "Shadowing and pattern-shadow diagnostics"),
    ("all", "Enable all debug subsystems"),
];

const KNOWN_FLAGS: [&str; 35] = [
    "--help", "-h", "--version", "-V", "--color", "--check", "--diag-json", "--dump", "--dump-ast",
    "--profile-compiler", "--assembly", "--out-dir", "--target-profile", "--opt-level", "--test",
    "--coverage", "--build-progress", "--incremental", "--max-errors", "--no-crash-report",
    "--runtime-lib", "--link-debug", "--no-link-debug", "--log", "--log-file", "--trace",
    "--trace-filter", "--trace-level", "--debug", "--conformance", "--emit-ir", "--phase1-only",
    "--no-output", "--verbose", "-v",
];

const BAD_TARGET_PROFILE: &str =
    "invalid target profile; expected one of: x86_64-sysv, x86_64-win64, aarch64-aapcs64, aarch64-darwin";
const BAD_OPT_LEVEL: &str = "invalid --opt-level value; expected one of: O0, O1, O2, O3, Os, Oz";
const BAD_COLOR: &str = "invalid --color value; expected 'auto', 'always', or 'never'";
const BAD_TRACE_FILTER: &str =
    "invalid --trace-filter value; expected comma-separated classes from log,diagnostic,runtime,all";
const BAD_TRACE_LEVEL: &str = "invalid --trace-level value; expected trace, info, warning, or error";
const BAD_MAX_ERRORS: &str = "invalid --max-errors value; expected non-negative integer or inf";

fn usage_error<T>(message: impl Into<String>) -> Result<T, CliError> {
    Err(CliError { message: message.into(), code: None })
}

fn parse_toggle_mode(value: &str) -> Option<bool> {
    match value {
        "on" | "true" | "TRUE" | "1" | "yes" | "YES" => Some(true),
        "off" | "false" | "FALSE" | "0" | "no" | "NO" => Some(false),
        _ => None,
    }
}

fn normalize_opt_level(value: &str) -> Option<&'static str> {
    match value {
        "O0" | "0" => Some("O0"),
        "O1" | "1" => Some("O1"),
        "O2" | "2" | "release" => Some("O2"),
        "O3" | "3" => Some("O3"),
        "Os" | "s" => Some("Os"),
        "Oz" | "z" => Some("Oz"),
        _ => None,
    }
}

/// Comma-separated tokens with surrounding spaces removed; empty tokens are dropped.
fn parse_csv_tokens(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|token| token.trim_matches(' '))
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .collect()
}

fn validate_debug_subsystems(subsystems: &[String]) -> Option<String> {
    if subsystems.is_empty() {
        return Some("--debug requires a comma-separated list of subsystems or 'help'".to_string());
    }
    let unknown = subsystems
        .iter()
        .find(|subsystem| !DEBUG_SUBSYSTEMS.iter().any(|(name, _)| name == subsystem))?;
    let names: Vec<&str> = DEBUG_SUBSYSTEMS.iter().map(|(name, _)| *name).collect();
    Some(format!("unknown debug subsystem '{unknown}'; expected one of: {}", names.join(", ")))
}

fn parse_trace_filter_mask(value: &str) -> Option<u8> {
    let classes = parse_csv_tokens(value);
    if classes.is_empty() {
        return None;
    }
    let mut mask = 0u8;
    for class in classes {
        match class.as_str() {
            "all" => mask = 0x7,
            "log" => mask |= 0x1,
            "diagnostic" | "diag" => mask |= 0x2,
            "runtime" => mask |= 0x4,
            _ => return None,
        }
    }
    Some(mask)
}

fn parse_trace_level(value: &str) -> Option<u8> {
    match value {
        "trace" => Some(0),
        "info" => Some(1),
        "warning" | "warn" => Some(2),
        "error" => Some(3),
        _ => None,
    }
}

fn parse_max_errors_policy(value: &str) -> Option<ErrorRecoveryPolicy> {
    if matches!(value, "inf" | "infinite" | "unlimited") {
        return Some(ErrorRecoveryPolicy { max_error_count: None });
    }
    if value.is_empty() || !value.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let parsed = value.parse::<usize>().ok()?;
    Some(ErrorRecoveryPolicy { max_error_count: Some(parsed) })
}

fn levenshtein_distance(a: &[u8], b: &[u8]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0usize; b.len() + 1];
    for i in 1..=a.len() {
        curr[0] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b.len()]
}

fn suggest_flag(unknown: &str) -> Option<&'static str> {
    let mut best = None;
    let mut best_dist = if unknown.len() <= 2 { 1 } else { 4 };
    for flag in KNOWN_FLAGS {
        let dist = levenshtein_distance(unknown.as_bytes(), flag.as_bytes());
        if dist < best_dist {
            best_dist = dist;
            best = Some(flag);
        }
    }
    best
}

/// Cursor over the arguments that resolves `--flag value` and `--flag=value` uniformly.
struct Args<'a> {
    args: &'a [String],
    index: usize,
}

impl<'a> Args<'a> {
    /// The value of option `name` when `arg` is `name` (value in the next argument) or
    /// `name=value`. `Ok(None)` means `arg` is a different option.
    fn value(&mut self, arg: &'a str, name: &str, missing: &str) -> Result<Option<&'a str>, CliError> {
        if arg == name {
            self.index += 1;
            return match self.args.get(self.index) {
                Some(value) => Ok(Some(value.as_str())),
                None => usage_error(missing),
            };
        }
        Ok(arg.strip_prefix(name).and_then(|rest| rest.strip_prefix('=')))
    }
}

fn non_empty(value: &str, message: &str) -> Result<String, CliError> {
    if value.is_empty() {
        return usage_error(message);
    }
    Ok(value.to_string())
}

/// Parses `argv` (including the program name at index 0).
pub fn parse_args(argv: &[String]) -> Result<CliOptions, CliError> {
    let mut opts = CliOptions::default();
    let mut command_selected = false;

    // Debug subsystems take effect before anything else is parsed.
    for (i, arg) in argv.iter().enumerate().skip(1) {
        let parsed = if arg == "--debug" {
            argv.get(i + 1).map(|value| parse_csv_tokens(value))
        } else if let Some(value) = arg.strip_prefix("--debug=") {
            Some(parse_csv_tokens(value))
        } else {
            continue;
        };
        if let Some(parsed) = parsed {
            if !(parsed.len() == 1 && parsed[0] == "help") {
                opts.debug_subsystems = parsed;
                set_debug_subsystems(&opts.debug_subsystems);
            }
        }
        break;
    }

    let mut cursor = Args { args: argv, index: 1 };
    while cursor.index < argv.len() {
        let arg = argv[cursor.index].as_str();
        let at = cursor.index;
        cursor.index += 1;
        let mut args = Args { args: argv, index: at };
        macro_rules! option {
            ($name:literal, $missing:literal) => {{
                let value = args.value(arg, $name, $missing)?;
                cursor.index = args.index + 1;
                value
            }};
        }
        match arg {
            "--help" | "-h" => {
                opts.show_help = true;
                return Ok(opts);
            }
            "--version" | "-V" => {
                opts.show_version = true;
                return Ok(opts);
            }
            "--diag-json" => opts.diag_json = true,
            "--profile-compiler" | "--profile-compiler=json" => opts.profile_compiler = true,
            "--verbose" | "-v" => opts.verbose = true,
            "--check" => opts.check_only = true,
            "--no-crash-report" => opts.no_crash_report = true,
            "--dump" => opts.dump_project = true,
            "--dump-ast" => opts.dump_ast = true,
            "--phase1-only" => opts.phase1_only = true,
            "--emit-ir" => opts.emit_ir = true,
            "--link-debug" => opts.link_debug = Some(true),
            "--no-link-debug" => opts.link_debug = Some(false),
            "--log" => {
                opts.log_enabled = true;
                opts.log_to_console = true;
            }
            "--trace" => {
                opts.log_enabled = true;
                opts.log_to_console = true;
                opts.trace = true;
            }
            "--no-output" => {
                if !has_debug_subsystems() {
                    return usage_error("--no-output requires --debug to be set");
                }
                opts.no_output = true;
            }
            "build" => command_selected = true,
            "test" => {
                command_selected = true;
                opts.do_test = true;
            }
            "init" => {
                command_selected = true;
                opts.do_init = true;
            }
            "clean" => {
                command_selected = true;
                opts.do_clean = true;
            }
            _ => {
                if arg.starts_with("--profile-compiler=") {
                    return usage_error("invalid --profile-compiler value; expected 'json'");
                }
                if let Some(value) =
                    option!("--target-profile", "--target-profile requires a profile argument")
                {
                    match parse_target_profile(value) {
                        Some(profile) => opts.target_profile_override = Some(profile),
                        None => return usage_error(BAD_TARGET_PROFILE),
                    }
                } else if let Some(value) =
                    option!("--opt-level", "--opt-level requires a value (O0|O1|O2|O3|Os|Oz)")
                {
                    match normalize_opt_level(value) {
                        Some(level) => opts.opt_level = Some(level.to_string()),
                        None => return usage_error(BAD_OPT_LEVEL),
                    }
                } else if let Some(value) =
                    option!("--color", "--color requires an argument (auto|always|never)")
                {
                    opts.color_mode = match value {
                        "auto" => ColorMode::Auto,
                        "always" => ColorMode::Always,
                        "never" => ColorMode::Never,
                        _ => return usage_error(BAD_COLOR),
                    };
                } else if let Some(value) =
                    option!("--conformance", "--conformance requires a path argument")
                {
                    opts.conformance_path = Some(value.to_string());
                } else if let Some(value) =
                    option!("--log-file", "--log-file requires a path argument")
                {
                    opts.log_file_path = Some(non_empty(value, "--log-file path must not be empty")?);
                    opts.log_enabled = true;
                    opts.log_to_file = true;
                } else if let Some(value) = option!(
                    "--trace-filter",
                    "--trace-filter requires classes (log,diagnostic,runtime,all)"
                ) {
                    let Some(mask) = parse_trace_filter_mask(value) else {
                        return usage_error(BAD_TRACE_FILTER);
                    };
                    opts.log_enabled = true;
                    opts.log_to_console = true;
                    opts.trace = true;
                    opts.trace_filter_mask = Some(mask);
                } else if let Some(value) = option!(
                    "--trace-level",
                    "--trace-level requires a level (trace|info|warning|error)"
                ) {
                    let Some(level) = parse_trace_level(value) else {
                        return usage_error(BAD_TRACE_LEVEL);
                    };
                    opts.log_enabled = true;
                    opts.log_to_console = true;
                    opts.trace = true;
                    opts.trace_min_level = Some(level);
                } else if let Some(value) =
                    option!("--build-progress", "--build-progress requires an argument (on|off)")
                {
                    let Some(enabled) = parse_toggle_mode(value) else {
                        return usage_error("invalid --build-progress value; expected 'on' or 'off'");
                    };
                    opts.build_progress = Some(enabled);
                } else if let Some(value) =
                    option!("--incremental", "--incremental requires an argument (on|off)")
                {
                    let Some(enabled) = parse_toggle_mode(value) else {
                        return usage_error("invalid --incremental value; expected 'on' or 'off'");
                    };
                    opts.incremental = Some(enabled);
                } else if let Some(value) =
                    option!("--max-errors", "--max-errors requires a value (N|inf)")
                {
                    let Some(policy) = parse_max_errors_policy(value) else {
                        return usage_error(BAD_MAX_ERRORS);
                    };
                    opts.max_errors_override = Some(policy);
                } else if let Some(value) =
                    option!("--runtime-lib", "--runtime-lib requires a path argument")
                {
                    opts.runtime_lib_path =
                        Some(non_empty(value, "--runtime-lib path must not be empty")?);
                } else if let Some(value) = arg.strip_prefix("--link-debug=") {
                    let Some(enabled) = parse_toggle_mode(value) else {
                        return usage_error("invalid --link-debug value; expected 'on' or 'off'");
                    };
                    opts.link_debug = Some(enabled);
                } else if let Some(value) = option!("--out-dir", "--out-dir requires a path argument")
                {
                    opts.out_dir = Some(non_empty(value, "--out-dir path must not be empty")?);
                } else if let Some(value) = option!(
                    "--debug",
                    "--debug requires a comma-separated list of subsystems or 'help'"
                ) {
                    let parsed = parse_csv_tokens(value);
                    if parsed.len() == 1 && parsed[0] == "help" {
                        opts.show_debug_help = true;
                        return Ok(opts);
                    }
                    if let Some(error) = validate_debug_subsystems(&parsed) {
                        return usage_error(error);
                    }
                    opts.debug_subsystems = parsed;
                    set_debug_subsystems(&opts.debug_subsystems);
                } else if let Some(value) =
                    option!("--assembly", "--assembly requires a name argument")
                {
                    opts.assembly_target = Some(value.to_string());
                } else if let Some(value) = option!(
                    "--test-harness-assembly",
                    "--test-harness-assembly requires a name argument"
                ) {
                    opts.test_harness_assembly = Some(value.to_string());
                } else if let Some(value) = option!(
                    "--test-harness-module",
                    "--test-harness-module requires a module path argument"
                ) {
                    opts.test_harness_module = Some(value.to_string());
                } else if let Some(value) = option!(
                    "--test-harness-dir",
                    "--test-harness-dir requires a path argument"
                ) {
                    opts.test_harness_dir = Some(value.to_string());
                } else if let Some(value) = option!("--test", "--test requires a name argument") {
                    opts.test_name_filter = Some(value.to_string());
                } else if let Some(value) =
                    option!("--coverage", "--coverage requires an obligation anchor argument")
                {
                    opts.test_coverage_filter = Some(value.to_string());
                } else if arg.starts_with('-') {
                    let mut message = format!("unknown option: {arg}");
                    if let Some(suggestion) = suggest_flag(arg) {
                        message.push_str(&format!("; did you mean '{suggestion}'?"));
                    }
                    return usage_error(message);
                } else if !command_selected {
                    return Err(CliError {
                        message: "unknown command".to_string(),
                        code: Some("E-CLI-0001"),
                    });
                } else if opts.do_test {
                    if opts.test_target.is_some() {
                        opts.test_target_rejected = true;
                    } else {
                        opts.test_target = Some(arg.to_string());
                    }
                } else if !opts.input_path.is_empty() {
                    return usage_error(format!(
                        "multiple input paths not supported; got '{}' and '{arg}'",
                        opts.input_path
                    ));
                } else {
                    opts.input_path = arg.to_string();
                }
            }
        }
    }
    if opts.do_init || opts.do_clean {
        if opts.input_path.is_empty() {
            opts.input_path = ".".to_string();
        }
        return Ok(opts);
    }
    if opts.do_test {
        opts.input_path = ".".to_string();
        return Ok(opts);
    }
    if opts.input_path.is_empty() {
        return usage_error("no input file specified");
    }
    Ok(opts)
}

pub fn escape_json(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

pub fn severity_string(severity: Severity) -> &'static str {
    severity.label()
}

fn sub_diag_kind_string(kind: SubDiagnosticKind) -> &'static str {
    match kind {
        SubDiagnosticKind::Note => "note",
        SubDiagnosticKind::Help => "help",
        SubDiagnosticKind::FixIt => "fix",
    }
}

fn span_json(sp: &Span) -> String {
    format!(
        "{{\"file\":\"{}\",\"start_line\":{},\"start_col\":{},\"end_line\":{},\"end_col\":{}}}",
        escape_json(&sp.file),
        sp.start_line,
        sp.start_col,
        sp.end_line,
        sp.end_col
    )
}

fn children_json(children: &[SubDiagnostic]) -> String {
    let rendered: Vec<String> = children
        .iter()
        .map(|child| {
            let mut out = format!(
                "{{\"kind\":\"{}\",\"message\":\"{}\",\"span\":{}",
                sub_diag_kind_string(child.kind),
                escape_json(&child.message),
                child.span.as_ref().map_or_else(|| "null".to_string(), span_json)
            );
            if let Some(fix_text) = &child.fix_text {
                out.push_str(&format!(",\"fix_text\":\"{}\"", escape_json(fix_text)));
            }
            out.push('}');
            out
        })
        .collect();
    format!("\"children\":[{}]", rendered.join(","))
}

/// The text of 1-based line `line_number`, without its line terminator.
fn source_line(source: &str, line_number: usize) -> &str {
    if line_number == 0 {
        return "";
    }
    let Some(line) = source.split('\n').nth(line_number - 1) else {
        return "";
    };
    // A final empty segment after a trailing newline is not a line.
    line.strip_suffix('\r').unwrap_or(line)
}

/// Maps a file path to its text.
pub type SourceLookup<'a> = &'a mut dyn FnMut(&str) -> Option<String>;

/// `sources` supplies the `source_line` field.
pub fn diagnostic_to_json(diag: &Diagnostic, sources: Option<SourceLookup<'_>>) -> String {
    let code = if diag.code.is_empty() {
        "null".to_string()
    } else {
        format!("\"{}\"", escape_json(&diag.code))
    };
    let (span, line) = match &diag.span {
        None => ("null".to_string(), "null".to_string()),
        Some(sp) => {
            let line = sources
                .and_then(|lookup| lookup(&sp.file))
                .map_or_else(
                    || "null".to_string(),
                    |content| format!("\"{}\"", escape_json(source_line(&content, sp.start_line))),
                );
            (span_json(sp), line)
        }
    };
    format!(
        "{{\"code\":{code},\"severity\":\"{}\",\"message\":\"{}\",\"span\":{span},\"source_line\":{line},{}}}",
        severity_string(diag.severity),
        escape_json(&diag.message),
        children_json(&diag.children)
    )
}

pub fn diagnostic_stream_to_json(
    stream: &[Diagnostic],
    mut sources: Option<SourceLookup<'_>>,
) -> String {
    let rendered: Vec<String> = stream
        .iter()
        .map(|diag| match sources.as_mut() {
            Some(lookup) => diagnostic_to_json(diag, Some(&mut **lookup)),
            None => diagnostic_to_json(diag, None),
        })
        .collect();
    format!("{{\"diagnostics\":[{}]}}", rendered.join(","))
}

pub fn print_usage(command_name: &str) {
    eprintln!(
        "usage: {command_name} <command> [options]\n       {command_name} build <file> [options]\n       {command_name} test [target] [options]\n       {command_name} init [directory]\n       {command_name} clean [file]\nTry '{command_name} --help' for more information."
    );
}

pub fn print_help(command_name: &str) {
    print!("{}", include_str!("help.txt").replace("{command}", command_name));
}

pub fn print_debug_help() {
    println!("ultraviolet debug subsystems\n");
    println!("Use --debug <name[,name...]> to enable subsystem traces.\n");
    for (name, description) in DEBUG_SUBSYSTEMS {
        println!("  {name:<9}  {description}");
    }
}
