//! Emits the same dumps as the reference oracle (`tools/oracle/oracle_main.cpp`) so the
//! two implementations can be compared byte for byte.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write as _;

use uv_core::diagnostics::Diagnostic;
use uv_core::source_load::load_source;
use uv_core::span::Span;
use uv_core::unicode::{analyze_identifier_security, case_fold, is_xid_continue, is_xid_start, nfc};
use uv_comptime::{execute_comptime, ComptimePassOptions};
use uv_analysis::context::{Entity, NameMapTable, Scope, ScopeContext, TypeDecl};
use uv_analysis::resolve::collect_toplevel::collect_name_maps;
use uv_analysis::resolve::populate_sigma::populate_sigma;
use uv_analysis::resolve::resolve_module::resolve_modules;
use uv_analysis::resolve::resolver::ResolveContext;
use uv_analysis::resolve::scopes_lookup::module_names_of;
use uv_analysis::resolve::visibility::{can_access, check_module_visibility};
use uv_core::diagnostics::{emit, has_error};
use uv_core::symbols::string_of_path;
use uv_project::target_profile::TargetProfile;
use uv_source::ast::ASTModule;
use uv_project::load_project::load_project;
use uv_project::manifest::find_project_root;
use uv_project::module_discovery::compilation_unit;
use uv_project::project::{Assembly, AssemblyTarget};
use uv_source::ast::dump::AstDump;
use uv_source::phase1::{run_phase1, AssemblyOutcome, Phase1Observer};
use uv_source::lexer::tokenize_with_diagnostics;
use uv_source::parser::parse_file;

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out
}

fn span_text(span: &Span) -> String {
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        span.start_offset, span.end_offset, span.start_line, span.start_col, span.end_line, span.end_col
    )
}

fn print_diags(out: &mut String, diags: &[Diagnostic]) {
    for diag in diags {
        let span = diag.span.as_ref().map_or_else(|| "-".to_string(), span_text);
        let _ = writeln!(
            out,
            "G\t{}\t{}\t{}\t{}\t{}",
            diag.code,
            diag.severity.label(),
            escape(&diag.message),
            span,
            diag.obligation_ids.join(",")
        );
    }
}

fn dump_tokens(out: &mut String, label: &str, path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|err| format!("cannot read {path}: {err}"))?;
    let _ = writeln!(out, "F\t{label}");
    let loaded = load_source(label, &bytes);
    print_diags(out, &loaded.diags);
    let Some(source) = loaded.source else {
        out.push_str("NOSOURCE\n");
        return Ok(());
    };
    let result = tokenize_with_diagnostics(&source);
    print_diags(out, &result.diags);
    let Some(output) = result.output else {
        out.push_str("NOTOKENS\n");
        return Ok(());
    };
    for token in &output.tokens {
        let _ = writeln!(
            out,
            "T\t{}\t{}\t{}",
            token.kind.name(),
            escape(&token.lexeme),
            span_text(&token.span)
        );
    }
    for doc in &output.docs {
        let _ = writeln!(out, "D\t{}\t{}\t{}", doc.kind.name(), escape(&doc.text), span_text(&doc.span));
    }
    Ok(())
}

fn dump_ast(out: &mut String, label: &str, path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|err| format!("cannot read {path}: {err}"))?;
    let _ = writeln!(out, "F\t{label}");
    let loaded = load_source(label, &bytes);
    print_diags(out, &loaded.diags);
    let Some(source) = loaded.source else {
        out.push_str("NOSOURCE\n");
        return Ok(());
    };
    let parsed = parse_file(&source);
    print_diags(out, &parsed.diags);
    for span in &parsed.unsafe_spans {
        let _ = writeln!(out, "U\t{}", span_text(span));
    }
    let Some(file) = parsed.file else {
        out.push_str("NOFILE\n");
        return Ok(());
    };
    for doc in &file.module_doc {
        out.push_str("M\t");
        doc.dump(out);
        out.push('\n');
    }
    for item in &file.items {
        out.push_str("I\t");
        item.dump(out);
        out.push('\n');
    }
    Ok(())
}

fn opt_text(text: &Option<String>) -> String {
    text.as_deref().map_or_else(|| "-".to_string(), escape)
}

fn file_span_text(span: &Option<Span>) -> String {
    span.as_ref().map_or_else(|| "-".to_string(), |span| format!("{}\t{}", span.file, span_text(span)))
}

/// Diagnostics with the span's file and the attached notes; see `PrintDiagsFull` in the oracle.
fn print_diags_full(out: &mut String, diags: &[Diagnostic]) {
    for diag in diags {
        let _ = writeln!(
            out,
            "G\t{}\t{}\t{}\t{}\t{}\t{}",
            diag.code,
            diag.severity.label(),
            escape(&diag.message),
            file_span_text(&diag.span),
            diag.obligation_ids.join(","),
            opt_text(&diag.label)
        );
        for child in &diag.children {
            let _ = writeln!(
                out,
                "N\t{}\t{}\t{}\t{}\t{}",
                child.kind as i32,
                escape(&child.message),
                file_span_text(&child.span),
                opt_text(&child.fix_text),
                opt_text(&child.label)
            );
        }
    }
}

/// One project block of a project list; see `LoadProjectBlock` in the oracle.
#[derive(Default)]
struct ProjectBlock {
    label: String,
    options: ComptimePassOptions,
    modules: Vec<ASTModule>,
    reachable: usize,
    unsafe_spans_by_file: HashMap<String, Vec<Span>>,
}

/// Loads a block, or returns the lines to print when one of its files cannot be used.
fn load_project_block(block: &[&str]) -> Result<ProjectBlock, String> {
    let mut out = ProjectBlock::default();
    for line in block {
        let fields: Vec<&str> = line.split('\t').collect();
        match fields.as_slice() {
            ["P", name, root, fallback, ..] => {
                out.label = name.to_string();
                out.options.project_root = root.to_string();
                out.options.fallback_source_root = Some(fallback.to_string());
            }
            ["A", name, source_root, ..] => {
                out.options.source_roots_by_assembly.insert(name.to_string(), source_root.to_string());
            }
            ["R", count, ..] => out.reachable = count.parse().unwrap_or(0),
            ["M", path, files @ ..] => {
                let mut module = ASTModule { path: path.split("::").map(str::to_string).collect(), ..ASTModule::default() };
                for file in files {
                    let bytes = std::fs::read(file).unwrap_or_default();
                    let Some(source) = load_source(file, &bytes).source else {
                        return Err(format!("F\t{}\nNOSOURCE\t{file}\n", out.label));
                    };
                    let parsed = parse_file(&source);
                    let Some(parsed_file) = parsed.file else {
                        return Err(format!("F\t{}\nNOFILE\t{file}\n", out.label));
                    };
                    module.items.extend(parsed_file.items);
                    module.module_doc.extend(parsed_file.module_doc);
                    out.unsafe_spans_by_file.insert(source.path.to_string(), parsed.unsafe_spans);
                }
                out.modules.push(module);
            }
            _ => {}
        }
    }
    Ok(out)
}

fn dump_line(tag: &str, value: &dyn AstDump, out: &mut String) {
    out.push_str(tag);
    out.push('\t');
    value.dump(out);
    out.push('\n');
}

fn dump_modules(out: &mut String, modules: &[ASTModule]) {
    for module in modules {
        dump_line("X", &module.path, out);
        for doc in &module.module_doc {
            dump_line("M", doc, out);
        }
        for item in &module.items {
            dump_line("I", item, out);
        }
        for proc in &module.comptime_procedures {
            dump_line("C", proc, out);
        }
    }
}

/// Runs the compile-time pass on one project block of a comptime list.
fn dump_comptime(out: &mut String, block: &[&str]) {
    let ProjectBlock { label, options, modules, .. } = match load_project_block(block) {
        Ok(project) => project,
        Err(failure) => {
            out.push_str(&failure);
            return;
        }
    };
    let _ = writeln!(out, "F\t{label}");
    let result = execute_comptime(&modules, &options);
    print_diags_full(out, &result.diags);
    let Some(expanded) = result.modules else {
        out.push_str("NOMODULES\n");
        return;
    };
    dump_modules(out, &expanded);
}

fn dump_name_maps(out: &mut String, table: &NameMapTable) {
    for (module_key, name_map) in table {
        dump_line("NM", module_key, out);
        let mut entries: Vec<(&String, &Entity)> = name_map.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        for (name, ent) in entries {
            out.push_str("NE\t");
            name.dump(out);
            let _ = write!(out, "\t{:?}\t{:?}\t", ent.kind, ent.source);
            ent.origin_opt.dump(out);
            out.push('\t');
            ent.target_opt.dump(out);
            out.push('\t');
            ent.declaration_span.dump(out);
            out.push('\t');
            ent.language_symbol_id.dump(out);
            out.push('\t');
            ent.type_param_class_bounds.dump(out);
            out.push('\t');
            ent.visibility.dump(out);
            out.push('\n');
        }
    }
}

/// Runs the front end through name resolution on one project block, in the order the
/// driver does; see `DumpResolve` in the oracle.
fn dump_resolve(out: &mut String, block: &[&str]) {
    let input = match load_project_block(block) {
        Ok(project) => project,
        Err(failure) => {
            out.push_str(&failure);
            return;
        }
    };
    let _ = writeln!(out, "F\t{}", input.label);
    let Some(project) = load_project(&input.options.project_root, &AssemblyTarget::default()).project else {
        out.push_str("NOPROJECT\n");
        return;
    };
    let comptime = execute_comptime(&input.modules, &input.options);
    let Some(mut parsed_modules) = comptime.modules.filter(|_| !has_error(&comptime.diags)) else {
        out.push_str("STOP\tcomptime\n");
        return;
    };
    let mut diags = Vec::new();
    for diag in comptime.diags {
        emit(&mut diags, diag);
    }
    parsed_modules.truncate(input.reachable);

    let mut sema_project = project.clone();
    sema_project.modules = parsed_modules
        .iter()
        .flat_map(|module| {
            let path = string_of_path(&module.path);
            project.assemblies.iter().flat_map(move |assembly| {
                let path = path.clone();
                assembly.modules.iter().filter(move |info| info.path == path).cloned()
            })
        })
        .collect();

    let mut ctx = ScopeContext {
        project: Some(&sema_project),
        target_profile: Some(TargetProfile::X86_64SysV),
        scopes: vec![Scope::new(), Scope::new(), Scope::new()],
        ..Default::default()
    };
    ctx.sigma.mods = parsed_modules;
    ctx.sigma.unsafe_spans_by_file = input.unsafe_spans_by_file;
    for index in 0..ctx.sigma.mods.len() {
        ctx.current_module = ctx.sigma.mods[index].path.clone();
        for diag in check_module_visibility(&ctx, &ctx.sigma.mods[index]) {
            emit(&mut diags, diag);
        }
    }
    let name_maps = collect_name_maps(&mut ctx);
    for diag in name_maps.diags {
        emit(&mut diags, diag);
    }
    print_diags_full(out, &diags);
    dump_name_maps(out, &name_maps.name_maps);
    if has_error(&diags) {
        out.push_str("STOP\tnames\n");
        return;
    }
    populate_sigma(&mut ctx);
    let module_names = module_names_of(&sema_project);
    let no_parse_diags = Vec::new();
    let mut res_ctx = ResolveContext {
        ctx: &mut ctx,
        name_maps: &name_maps.name_maps,
        module_names: &module_names,
        can_access: Some(can_access),
        parse_ok: true,
        parse_diags: Some(&no_parse_diags),
    };
    let resolved = resolve_modules(&mut res_ctx);
    let _ = writeln!(out, "RESOLVE\t{}", if resolved.ok { "ok" } else { "failed" });
    print_diags_full(out, &resolved.diags);
    dump_modules(out, &resolved.modules);
}

/// The built-in declarations; see `DumpSigma` in the oracle.
fn dump_sigma(out: &mut String) {
    let mut ctx = ScopeContext::default();
    populate_sigma(&mut ctx);
    for (key, decl) in &ctx.sigma.types {
        out.push_str("T\t");
        key.dump(out);
        out.push('\t');
        match decl {
            TypeDecl::Record(node) => node.dump(out),
            TypeDecl::Enum(node) => node.dump(out),
            TypeDecl::Modal(node) => node.dump(out),
            TypeDecl::TypeAlias(node) => node.dump(out),
        }
        out.push('\n');
    }
    for (key, decl) in &ctx.sigma.classes {
        out.push_str("K\t");
        key.dump(out);
        out.push('\t');
        decl.dump(out);
        out.push('\n');
    }
}

struct Quiet;

impl Phase1Observer for Quiet {
    fn assembly_start(&self, _: &Assembly) {}
    fn assembly_finish(&self, _: &Assembly, _: AssemblyOutcome) {}
}

/// Writes the comptime list block for one manifest, or nothing when the project does not
/// pass phase 1. See `DumpComptime` in the oracle for the format.
fn comptime_list_block(out: &mut String, manifest: &str) {
    let root = find_project_root(manifest);
    let loaded = load_project(&root, &AssemblyTarget::default());
    let Some(project) = loaded.project.filter(|_| !has_error(&loaded.diags)) else {
        return;
    };
    let phase1 = run_phase1(&project, &Quiet);
    if !phase1.ok || has_error(&phase1.diags) {
        return;
    }
    let _ = writeln!(out, "P\t{manifest}\t{}\t{}", project.root, project.source_root);
    for assembly in &project.assemblies {
        let _ = writeln!(out, "A\t{}\t{}", assembly.name, assembly.source_root);
    }
    let _ = writeln!(out, "R\t{}", phase1.reachable_count);
    for info in &phase1.project_module_infos {
        let _ = write!(out, "M\t{}", info.path);
        for file in compilation_unit(&info.dir).files {
            let _ = write!(out, "\t{file}");
        }
        out.push('\n');
    }
    out.push_str("E\n");
}

fn dump_unicode(out: &mut impl std::io::Write) -> std::io::Result<()> {
    for c in 0..=0x10FFFFu32 {
        let Some(ch) = char::from_u32(c) else {
            continue;
        };
        let text = ch.to_string();
        let xid_start = is_xid_start(c);
        let xid_continue = is_xid_continue(c);
        let normalized = nfc(&text);
        let folded = case_fold(&text);
        let probe = format!("a{text}");
        let info = analyze_identifier_security(&probe);
        let trivial = !xid_start
            && !xid_continue
            && normalized == text
            && folded == text
            && info.skeleton == probe
            && !info.mixed_script;
        if trivial {
            continue;
        }
        writeln!(
            out,
            "{:X}\t{}\t{}\t{}\t{}\t{}\t{}",
            c,
            xid_start as u8,
            xid_continue as u8,
            escape(&normalized),
            escape(&folded),
            escape(&info.skeleton),
            info.mixed_script as u8
        )?;
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout.lock());
    match args.get(1).map(String::as_str) {
        Some("unicode") => dump_unicode(&mut stdout).map_err(|err| err.to_string()),
        Some(mode @ ("tokens" | "ast")) if args.len() >= 3 => {
            // The list file holds one "label<TAB>path" per line. `--root` rewrites the
            // container-side `/w/` prefix used when the list was written for the oracle.
            let root = args.iter().position(|a| a == "--root").and_then(|i| args.get(i + 1));
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            for line in list.lines() {
                let Some((label, path)) = line.split_once('\t') else {
                    continue;
                };
                let path = match (root, path.strip_prefix("/w/")) {
                    (Some(root), Some(rest)) => format!("{root}/{rest}"),
                    _ => path.to_string(),
                };
                let mut out = String::new();
                if mode == "ast" {
                    dump_ast(&mut out, label, &path)?;
                } else {
                    dump_tokens(&mut out, label, &path)?;
                }
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
                if mode == "ast" {
                    stdout.flush().map_err(|err| err.to_string())?;
                }
            }
            Ok(())
        }
        Some("sigma") => {
            let mut out = String::new();
            dump_sigma(&mut out);
            stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())
        }
        Some(mode @ ("comptime" | "resolve")) if args.len() >= 3 => {
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            let mut block: Vec<&str> = Vec::new();
            for line in list.lines() {
                if line != "E" {
                    block.push(line);
                    continue;
                }
                let mut out = String::new();
                if mode == "resolve" {
                    dump_resolve(&mut out, &block);
                } else {
                    dump_comptime(&mut out, &block);
                }
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
                block.clear();
            }
            Ok(())
        }
        Some("comptime-list") if args.len() >= 3 => {
            // The argument lists manifests, one per line, relative to the working directory.
            let cwd = std::env::current_dir().map_err(|err| err.to_string())?;
            let list = std::fs::read_to_string(&args[2]).map_err(|err| err.to_string())?;
            for manifest in list.lines().filter(|line| !line.is_empty()) {
                let mut out = String::new();
                comptime_list_block(&mut out, &cwd.join(manifest).to_string_lossy());
                stdout.write_all(out.as_bytes()).map_err(|err| err.to_string())?;
            }
            Ok(())
        }
        _ => Err("usage: uv-parity unicode | tokens <list-file> [--root <dir>] | ast <list-file> [--root <dir>]".to_string()),
    }
}

fn main() {
    // Deeply nested inputs recurse far in the parser; the reference runs with an 8 MiB
    // main-thread stack and larger frames, so give the port generous room.
    let worker = std::thread::Builder::new().stack_size(1 << 30).spawn(run).expect("spawn worker");
    match worker.join() {
        Ok(Ok(())) => {}
        Ok(Err(message)) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
        Err(_) => std::process::exit(101),
    }
}
