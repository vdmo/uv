//! `uv-lsp`: the Ultraviolet language server over standard input and output.

mod completion;
mod config;
mod diagnostics;
mod handlers;
mod handlers_edit;
mod hierarchy;
mod jsonx;
mod modules;
mod navigation;
mod semantic_tokens;
mod server;
mod symbols;
mod text;

use server::{LspServerOptions, Server};
use uv_project::target_profile::parse_target_profile;

fn print_usage() {
    println!(
        "Ultraviolet language server\n\nUSAGE\n  uv-lsp [--stdio] [--log-file <path>] [--target-profile <profile>]\n  uv-lsp --version\n\nOPTIONS\n  --stdio           Run LSP over standard input/output (default)\n  --log-file <path> Write server lifecycle logs to <path>\n  --target-profile <profile>\n                    Select target profile: x86_64-sysv,\n                    x86_64-win64, aarch64-aapcs64,\n                    aarch64-darwin\n  --version         Print version and exit\n  -h, --help        Show this help message"
    );
}

fn run() -> i32 {
    let mut options = LspServerOptions::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--stdio" => {}
            "--version" => {
                println!("Ultraviolet {} language server", env!("CARGO_PKG_VERSION"));
                return 0;
            }
            "-h" | "--help" => {
                print_usage();
                return 0;
            }
            "--log-file" => {
                let Some(path) = args.next() else {
                    eprintln!("uv-lsp: --log-file requires a path");
                    return 2;
                };
                options.log_file = Some(path.into());
            }
            "--target-profile" => {
                let Some(value) = args.next() else {
                    eprintln!("uv-lsp: --target-profile requires a profile");
                    return 2;
                };
                match parse_target_profile(&value) {
                    Some(profile) => options.target_profile = Some(profile),
                    None => {
                        eprintln!("uv-lsp: unknown target profile: {value}");
                        return 2;
                    }
                }
            }
            other => {
                eprintln!("uv-lsp: unknown option: {other}");
                return 2;
            }
        }
    }
    Server::new(options).run()
}

fn main() {
    // Analysis recurses deeply on nested input; give the main thread the room the parser gets.
    let worker = std::thread::Builder::new().stack_size(1 << 30).spawn(run).expect("spawn the server thread");
    std::process::exit(worker.join().unwrap_or(1));
}
