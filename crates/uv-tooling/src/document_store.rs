//! The documents the editor has open, whose text stands in for the files.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::uri::{normalize_path, path_key};

#[derive(Debug, Clone)]
pub struct DocumentOverlay {
    pub uri: String,
    pub path: PathBuf,
    pub version: i64,
    pub text_utf8: String,
}

#[derive(Default)]
pub struct DocumentStore {
    by_uri: HashMap<String, DocumentOverlay>,
}

impl DocumentStore {
    pub fn open(&mut self, uri: &str, path: &Path, version: i64, text_utf8: String) {
        self.by_uri.insert(uri.to_string(), DocumentOverlay { uri: uri.to_string(), path: normalize_path(path), version, text_utf8 });
    }

    pub fn change_full(&mut self, uri: &str, version: i64, text_utf8: String) -> bool {
        match self.by_uri.get_mut(uri) {
            Some(document) => {
                document.version = version;
                document.text_utf8 = text_utf8;
                true
            }
            None => false,
        }
    }

    pub fn close(&mut self, uri: &str) -> bool {
        self.by_uri.remove(uri).is_some()
    }

    pub fn overlays(&self) -> Vec<DocumentOverlay> {
        self.by_uri.values().cloned().collect()
    }

    pub fn find_by_uri(&self, uri: &str) -> Option<&DocumentOverlay> {
        self.by_uri.get(uri)
    }

    pub fn find_by_path(&self, path: &Path) -> Option<&DocumentOverlay> {
        let key = path_key(path);
        self.by_uri.values().find(|document| path_key(&document.path) == key)
    }
}
