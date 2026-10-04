//! Host-independent path algebra from the project model (both `/` and `\` separate).

use crate::spec_rule;

fn is_win_sep(c: u8) -> bool {
    c == b'/' || c == b'\\'
}

pub fn drive_rooted(p: &str) -> bool {
    let b = p.as_bytes();
    b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && is_win_sep(b[2])
}

pub fn unc(p: &str) -> bool {
    p.starts_with("//") || p.starts_with("\\\\")
}

pub fn root_relative(p: &str) -> bool {
    !p.is_empty() && is_win_sep(p.as_bytes()[0]) && !unc(p) && !drive_rooted(p)
}

pub fn abs_path(p: &str) -> bool {
    drive_rooted(p) || unc(p) || root_relative(p)
}

pub fn is_relative(p: &str) -> bool {
    !abs_path(p)
}

pub fn root_tag(p: &str) -> String {
    if drive_rooted(p) {
        p[..2].to_string()
    } else if unc(p) {
        "//".to_string()
    } else if root_relative(p) {
        "/".to_string()
    } else {
        String::new()
    }
}

pub fn tail(p: &str) -> &str {
    if drive_rooted(p) {
        &p[3..]
    } else if unc(p) {
        &p[2..]
    } else if root_relative(p) {
        &p[1..]
    } else {
        p
    }
}

pub fn segs(p: &str) -> Vec<String> {
    p.split(['/', '\\']).filter(|seg| !seg.is_empty()).map(str::to_string).collect()
}

pub fn path_comps(p: &str) -> Vec<String> {
    let root = root_tag(p);
    if root.is_empty() {
        return segs(p);
    }
    let mut comps = vec![root];
    comps.extend(segs(tail(p)));
    comps
}

pub fn drop_comps(n: usize, comps: &[String]) -> Vec<String> {
    if n >= comps.len() {
        return Vec::new();
    }
    comps[n..].to_vec()
}

pub fn last_comp(comps: &[String]) -> Option<String> {
    comps.last().cloned()
}

pub fn join_comp(comps: &[String]) -> String {
    let mut out = String::new();
    for (index, c) in comps.iter().enumerate() {
        out.push_str(c);
        if index + 1 != comps.len() && c != "/" && c != "//" {
            out.push('/');
        }
    }
    out
}

pub fn join(a: &str, b: &str) -> String {
    if abs_path(b) {
        return b.to_string();
    }
    let mut comps = path_comps(a);
    comps.extend(path_comps(b));
    join_comp(&comps)
}

pub fn normalize(p: &str) -> String {
    let comps: Vec<String> = path_comps(p).into_iter().filter(|c| c != ".").collect();
    join_comp(&comps)
}

pub fn canon(p: &str) -> Option<String> {
    let norm = normalize(p);
    if path_comps(&norm).iter().any(|c| c == "..") {
        return None;
    }
    Some(norm)
}

pub fn path_prefix(path: &[String], pref: &[String]) -> bool {
    path.len() >= pref.len() && path[..pref.len()] == *pref
}

pub fn prefix(p: &str, q: &str) -> bool {
    path_prefix(&path_comps(q), &path_comps(p))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveResult {
    pub root: String,
    pub path: String,
}

pub fn resolve(root: &str, p: &str) -> Option<ResolveResult> {
    let joined = normalize(&join(root, p));
    match (canon(root), canon(&joined)) {
        (Some(root), Some(path)) => {
            spec_rule!("Resolve-Canonical");
            Some(ResolveResult { root, path })
        }
        _ => {
            spec_rule!("Resolve-Canonical-Err");
            None
        }
    }
}

pub fn relative(p: &str, base: &str) -> Option<String> {
    let canon_path = canon(p)?;
    let canon_base = canon(base)?;
    let path_comps_v = path_comps(&canon_path);
    let base_comps = path_comps(&canon_base);
    if !path_prefix(&path_comps_v, &base_comps) {
        return None;
    }
    Some(join_comp(&drop_comps(base_comps.len(), &path_comps_v)))
}

pub fn basename(p: &str) -> String {
    last_comp(&path_comps(p)).unwrap_or_default()
}

pub fn file_ext(p: &str) -> String {
    let base = basename(p);
    match base.rfind('.') {
        None | Some(0) => String::new(),
        Some(pos) => base[pos..].to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_and_normalizes() {
        assert_eq!(join("/a/b", "c/d"), "/a/b/c/d");
        assert_eq!(join("C:\\a", "b"), "C:/a/b");
        assert_eq!(join("/a", "/b"), "/b");
        assert_eq!(normalize("//host/./share/x"), "//host/share/x");
        assert_eq!(canon("a/../b"), None);
        assert_eq!(relative("/a/b/c", "/a"), Some("b/c".to_string()));
        assert_eq!(relative("/a/b", "/x"), None);
        assert_eq!(file_ext("Source/Main.uv"), ".uv");
        assert_eq!(file_ext(".hidden"), "");
        assert!(prefix("/a", "/a/b"));
    }
}
