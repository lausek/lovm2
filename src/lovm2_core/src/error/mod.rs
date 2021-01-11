mod compile;
mod methods;
mod ty;

pub use self::compile::Lovm2CompileError;
pub use self::methods::*;
pub use self::ty::Lovm2ErrorTy;

pub type Lovm2Result<T> = Result<T, Lovm2Error>;
pub type Lovm2CompileResult<T> = Result<T, Lovm2CompileError>;

// TODO: format output/display; only output first backtrace line that starts with `lovm2`
#[derive(Debug)]
pub struct Lovm2Error {
    pub ty: Lovm2ErrorTy,
    pub msg: String,
    pub trace: backtrace::Backtrace,
}

impl From<Lovm2ErrorTy> for Lovm2Error {
    fn from(ty: Lovm2ErrorTy) -> Self {
        Self {
            ty,
            ..Self::default()
        }
    }
}

impl From<std::io::Error> for Lovm2Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            ty: Lovm2ErrorTy::Custom(format!("{}", e)),
            ..Self::default()
        }
    }
}

impl <T> From<(Lovm2ErrorTy, T)> for Lovm2Error
where T: AsRef<str>
{
    fn from(descr: (Lovm2ErrorTy, T)) -> Self {
        Self {
            ty: descr.0,
            msg: descr.1.as_ref().to_string(),
            ..Self::default()
        }
    }
}

impl From<(String, String)> for Lovm2Error {
    fn from(f: (String, String)) -> Self {
        Self {
            ty: Lovm2ErrorTy::Custom(f.0),
            msg: f.1,
            ..Self::default()
        }
    }
}

/*
impl <T> From<T> for Lovm2Error
where T: std::fmt::Display
{
    fn from(val: T) -> Self {
        Self {
            msg: format!("{}", val),
            ..Self::default()
        }
    }
}
*/

impl From<String> for Lovm2Error {
    fn from(msg: String) -> Self {
        Self {
            msg,
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
        writeln!(f, "{}: {}", self.ty, self.msg)?;
        writeln!(f, "{:?}", self.trace)?;
        Ok(())
    }
}

impl Default for Lovm2Error {
    fn default() -> Self {
        Self {
            ty: Lovm2ErrorTy::Basic,
            msg: String::new(),
            trace: backtrace::Backtrace::new(),
        }
    }
}
