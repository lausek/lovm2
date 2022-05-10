use std::path::Path;

/// If a module was not assigned a name, take this instead.
pub const LV2_DEFAULT_MODULE_NAME: &str = "_unknown_";

/// Meta information required on native and shared object modules.
#[derive(Clone, Debug)]
pub struct LV2ModuleMeta {
    /// Location of the modules source.
    pub(crate) loc: Option<String>,
    /// Module name.
    pub(crate) name: String,
}

impl LV2ModuleMeta {
    pub fn new(name: String, loc: Option<String>) -> Self {
        Self { loc, name }
    }
}

impl From<&Path> for LV2ModuleMeta {
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

impl From<String> for LV2ModuleMeta {
    fn from(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl std::default::Default for LV2ModuleMeta {
    fn default() -> Self {
        Self {
            loc: None,
            name: LV2_DEFAULT_MODULE_NAME.to_string(),
        }
    }
}
