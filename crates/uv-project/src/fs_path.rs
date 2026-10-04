//! Generic (forward-slash) path strings with the joining rules of the reference host layer.

use std::path::Path;

/// Appends `tail` to `base` the way `std::filesystem::path::operator/` does.
pub fn fs_join(base: &str, tail: &str) -> String {
    if tail.is_empty() {
        return base.to_string();
    }
    if Path::new(tail).is_absolute() || base.is_empty() {
        return tail.to_string();
    }
    if base.ends_with('/') {
        format!("{base}{tail}")
    } else {
        format!("{base}/{tail}")
    }
}

/// Renders a host path in generic form.
pub fn generic_string(path: &Path) -> String {
    let text = path.to_string_lossy();
    if cfg!(windows) {
        text.replace('\\', "/")
    } else {
        text.into_owned()
    }
}

/// Parent directory in generic form; the root is its own parent.
pub fn parent_path(path: &str) -> String {
    match Path::new(path).parent() {
        Some(parent) => generic_string(parent),
        None => path.to_string(),
    }
}
