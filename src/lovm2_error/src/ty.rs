#[derive(Debug, PartialEq)]
pub enum Lovm2ErrorTy {
    Custom(String),

    Basic,
    FrameStackEmpty,
    ImportConflict,
    KeyNotFound,
    LookupFailed,
    ModuleNotFound,
    OperationNotSupported,
    ValueStackEmpty,
}

impl std::fmt::Display for Lovm2ErrorTy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Self::Custom(cty) = self {
            write!(f, "{}", cty)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
