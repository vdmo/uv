//! Zed extension for Ultraviolet: starts `uv-lsp` and passes it the target profile.
//!
//! The server is looked for in this order: `ULTRAVIOLET_LSP_SERVER`, `uv-lsp` on the
//! `PATH` of the worktree, and `target/release/uv-lsp` inside the worktree (what
//! `cargo build --release -p uv-lsp` produces when the workspace itself is open).

use zed_extension_api::{self as zed, serde_json::json, Result};

struct UltravioletExtension;

impl zed::Extension for UltravioletExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(&mut self, _language_server_id: &zed::LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command> {
        Ok(zed::Command { command: resolve_server_path(worktree)?, args: vec!["--stdio".to_string()], env: worktree.shell_env() })
    }

    fn language_server_workspace_configuration(&mut self, _language_server_id: &zed::LanguageServerId, worktree: &zed::Worktree) -> Result<Option<zed::serde_json::Value>> {
        Ok(env_var(worktree, "ULTRAVIOLET_TARGET_PROFILE").map(|profile| {
            json!({ "targetProfile": profile, "ultraviolet": { "languageServer": { "targetProfile": profile } } })
        }))
    }
}

fn resolve_server_path(worktree: &zed::Worktree) -> Result<String> {
    if let Some(path) = env_var(worktree, "ULTRAVIOLET_LSP_SERVER") {
        return Ok(path);
    }
    if let Some(path) = worktree.which("uv-lsp").or_else(|| worktree.which("uv-lsp.exe")) {
        return Ok(path);
    }
    let in_workspace = format!("{}/target/release/uv-lsp", worktree.root_path());
    if worktree.read_text_file("target/release/uv-lsp").is_ok() || std::path::Path::new(&in_workspace).exists() {
        return Ok(in_workspace);
    }
    Err("Ultraviolet language server not found. Set ULTRAVIOLET_LSP_SERVER, put uv-lsp on PATH, or build it with `cargo build --release -p uv-lsp`.".to_string())
}

fn env_var(worktree: &zed::Worktree, name: &str) -> Option<String> {
    worktree.shell_env().into_iter().find_map(|(key, value)| (key == name).then_some(value.trim().to_string()).filter(|value| !value.is_empty()))
}

zed::register_extension!(UltravioletExtension);
