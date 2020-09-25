pub type Lovm2Result<T> = Result<T, Lovm2Error>;
pub type Lovm2CompileResult<T> = Result<T, Lovm2CompileError>;

#[derive(Debug)]
pub enum Lovm2Error {
    Msg(Option<String>, String),
}

impl From<String> for Lovm2Error {
    fn from(f: String) -> Self {
        Self::Msg(None, f)
    }
}

impl From<&str> for Lovm2Error {
    fn from(f: &str) -> Self {
        Self::Msg(None, f.to_string())
    }
}

impl std::fmt::Display for Lovm2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Msg(Some(ty), msg) => write!(f, "{}: {}", ty, msg),
            Self::Msg(None, msg) => write!(f, "{}", msg),
        }
    }
}

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
