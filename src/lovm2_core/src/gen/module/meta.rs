use std::path::Path;

/// If a module was not assigned a name, take this instead.
pub const DEFAULT_MODULE_NAME: &str = "_unknown_";

/// Meta information required on native and shared object modules.
#[derive(Clone, Debug)]
pub struct ModuleMeta {
    /// Location of the modules source.
    pub(crate) loc: Option<String>,
    /// Module name.
    pub(crate) name: String,
    /// Other required modules to be included on load.
    pub(crate) uses: Vec<String>,
}

impl ModuleMeta {
    pub fn new(name: String, loc: Option<String>, uses: Vec<String>) -> Self {
        Self { loc, name, uses }
    }

    /// Overwrite the dependencies.
    pub fn set_uses(&mut self, uses: Vec<String>) {
        self.uses = uses;
    }
}

impl From<&Path> for ModuleMeta {
    fn from(path: &Path) -> Self {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let loc = Some(path.display().to_string());
        Self {
            loc,
            name,
            ..Self::default()
        }
    }
}

impl From<String> for ModuleMeta {
    fn from(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl std::default::Default for ModuleMeta {
    fn default() -> Self {
        Self {
            loc: None,
            name: DEFAULT_MODULE_NAME.to_string(),
            uses: vec![],
        }
    }
}
