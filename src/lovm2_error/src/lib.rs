pub type Lovm2Result<T> = Result<T, Lovm2Error>;
pub type Lovm2CompileResult<T> = Result<T, Lovm2CompileError>;

// TODO: make this a struct
#[derive(Debug)]
pub struct Lovm2Error {
    pub ty: Option<String>,
    pub msg: String,

    #[cfg(feature = "backtracing")]
    pub trace: backtrace::Backtrace,
}

impl From<(String, String)> for Lovm2Error {
    fn from(f: (String, String)) -> Self {
        Self {
            ty: Some(f.0),
            msg: f.1,

            #[cfg(feature = "backtracing")]
            trace: backtrace::Backtrace::new(),
        }
    }
}

impl From<String> for Lovm2Error {
    fn from(f: String) -> Self {
        Self {
            ty: None,
            msg: f,

            #[cfg(feature = "backtracing")]
            trace: backtrace::Backtrace::new(),
        }
    }
}

impl From<&str> for Lovm2Error {
    fn from(f: &str) -> Self {
        Self {
            ty: None,
            msg: f.to_string(),

            #[cfg(feature = "backtracing")]
            trace: backtrace::Backtrace::new(),
        }
    }
}

impl std::fmt::Display for Lovm2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature="backtracing")]
        {
            write!(f, "{:?}", self.trace)?;
        }

        if let Some(ty) = &self.ty {
            write!(f, "{}: {}", ty, self.msg)
        } else {
            write!(f, "{}", self.msg)
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
