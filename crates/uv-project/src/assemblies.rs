use crate::project::{Assembly, Project};

pub fn is_valid_assembly_kind(kind: &str) -> bool {
    matches!(kind, "executable" | "library" | "dependency")
}

pub fn is_valid_link_kind(link_kind: &str) -> bool {
    matches!(link_kind, "shared" | "static")
}

impl Assembly {
    pub fn is_executable(&self) -> bool {
        self.kind == "executable"
    }

    pub fn is_library(&self) -> bool {
        self.kind == "library"
    }

    pub fn is_dependency(&self) -> bool {
        self.kind == "dependency"
    }

    pub fn is_linkable(&self) -> bool {
        self.is_executable() || self.is_library()
    }

    pub fn is_shared_library(&self) -> bool {
        self.is_library() && self.link_kind.as_deref() == Some("shared")
    }

    pub fn is_static_library(&self) -> bool {
        self.is_library() && self.link_kind.as_deref() == Some("static")
    }
}

pub fn get_assembly_names(project: &Project) -> Vec<String> {
    project.assemblies.iter().map(|assembly| assembly.name.clone()).collect()
}

/// The assembly with the given name, unless the name is missing or ambiguous.
pub fn get_assembly_by_name(project: &Project, name: &str) -> Option<Assembly> {
    let mut matches = project.assemblies.iter().filter(|assembly| assembly.name == name);
    let first = matches.next()?;
    matches.next().is_none().then(|| first.clone())
}
