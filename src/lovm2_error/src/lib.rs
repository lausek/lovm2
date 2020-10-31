mod ty;

pub use crate::ty::Lovm2ErrorTy;

pub type Lovm2Result<T> = Result<T, Lovm2Error>;
pub type Lovm2CompileResult<T> = Result<T, Lovm2CompileError>;

// TODO: make this a struct
#[derive(Debug)]
pub struct Lovm2Error {
    pub ty: Option<Lovm2ErrorTy>,
    pub msg: String,

    #[cfg(feature = "backtracing")]
    pub trace: backtrace::Backtrace,
}

impl From<Lovm2ErrorTy> for Lovm2Error {
    fn from(ty: Lovm2ErrorTy) -> Self {
        Self {
            ty: Some(ty),
            ..Self::default()
        }
    }
}

impl <T> From<(Lovm2ErrorTy, T)> for Lovm2Error
where T: AsRef<str>
{
    fn from(descr: (Lovm2ErrorTy, T)) -> Self {
        Self {
            ty: Some(descr.0),
            msg: descr.1.as_ref().to_string(),
            ..Self::default()
        }
    }
}

impl From<(String, String)> for Lovm2Error {
    fn from(f: (String, String)) -> Self {
        Self {
            ty: Some(Lovm2ErrorTy::Custom(f.0)),
            msg: f.1,
            ..Self::default()
        }
    }
}

impl From<String> for Lovm2Error {
    fn from(f: String) -> Self {
        Self {
            msg: f,
            ..Self::default()
        }
    }
}

impl From<&str> for Lovm2Error {
    fn from(f: &str) -> Self {
        Self {
            msg: f.to_string(),
            ..Self::default()
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

impl Default for Lovm2Error {
    fn default() -> Self {
        Self {
            ty: None,
            msg: String::new(),

            #[cfg(feature = "backtracing")]
            trace: backtrace::Backtrace::new(),
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
