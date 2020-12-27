#[derive(Debug)]
pub enum Lovm2CompileError {
    Msg(Option<String>, String),
}

impl From<String> for Lovm2CompileError {
    fn from(f: String) -> Self {
        Self::Msg(None, f)
    }
}

impl From<&str> for Lovm2CompileError {
    fn from(f: &str) -> Self {
        Self::Msg(None, f.to_string())
    }
}

impl std::fmt::Display for Lovm2CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Msg(Some(ty), msg) => write!(f, "{}: {}", ty, msg),
            Self::Msg(None, msg) => write!(f, "{}", msg),
        }
    }
}
