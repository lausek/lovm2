#[derive(Debug, PartialEq)]
pub enum LV2ErrorTy {
    Custom(String),

    Basic,
    FrameStackEmpty,
    ImportConflict,
    InvalidSetTarget,
    InvalidType,
    KeyNotFound,
    LookupFailed,
    ModuleNotFound,
    NoEntryPoint,
    OperationNotSupported,
    ValueStackEmpty,
}

impl std::fmt::Display for LV2ErrorTy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Self::Custom(cty) = self {
            write!(f, "{}", cty)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
