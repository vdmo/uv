use crate::host::{host_get_env_utf8, query_host_terminal, HostStream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldMagenta,
    BoldCyan,
    BoldWhite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorOverride {
    /// Delegate to `is_color_enabled`.
    #[default]
    Auto,
    ForceOn,
    ForceOff,
}

pub fn is_color_enabled(stream: HostStream) -> bool {
    if host_get_env_utf8("NO_COLOR").is_some() {
        return false;
    }
    query_host_terminal(stream).ansi_enabled
}

pub fn is_color_enabled_with_override(stream: HostStream, override_mode: ColorOverride) -> bool {
    match override_mode {
        ColorOverride::ForceOn => true,
        ColorOverride::ForceOff => false,
        ColorOverride::Auto => is_color_enabled(stream),
    }
}

pub fn terminal_width() -> i32 {
    let info = query_host_terminal(HostStream::Stderr);
    if info.width > 0 {
        return info.width;
    }
    if let Some(columns) = host_get_env_utf8("COLUMNS") {
        // Mirrors atoi: leading whitespace, optional sign, then leading digits.
        let trimmed = columns.trim_start();
        let digits_end = trimmed
            .char_indices()
            .find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && (c == '+' || c == '-'))))
            .map_or(trimmed.len(), |(i, _)| i);
        if let Ok(value) = trimmed[..digits_end].parse::<i32>() {
            if value > 0 {
                return value;
            }
        }
    }
    0
}

pub fn color_code(color: Color, enabled: bool) -> &'static str {
    if !enabled {
        return "";
    }
    match color {
        Color::Reset => "\x1b[0m",
        Color::Red => "\x1b[31m",
        Color::Green => "\x1b[32m",
        Color::Yellow => "\x1b[33m",
        Color::Blue => "\x1b[34m",
        Color::Magenta => "\x1b[35m",
        Color::Cyan => "\x1b[36m",
        Color::White => "\x1b[37m",
        Color::BoldRed => "\x1b[1;31m",
        Color::BoldGreen => "\x1b[1;32m",
        Color::BoldYellow => "\x1b[1;33m",
        Color::BoldBlue => "\x1b[1;34m",
        Color::BoldMagenta => "\x1b[1;35m",
        Color::BoldCyan => "\x1b[1;36m",
        Color::BoldWhite => "\x1b[1;37m",
    }
}

pub fn colorize(text: &str, color: Color, enabled: bool) -> String {
    if !enabled {
        return text.to_string();
    }
    format!("{}{text}{}", color_code(color, true), color_code(Color::Reset, true))
}
