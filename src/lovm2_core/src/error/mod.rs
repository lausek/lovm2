//! Error values for compilation and runtime

mod compile;
mod methods;
mod ty;

pub use self::compile::LV2CompileError;
pub use self::methods::*;
pub use self::ty::LV2ErrorTy;

/// Runtime Result
pub type LV2Result<T> = Result<T, LV2Error>;
/// Compile Result
pub type LV2CompileResult<T> = Result<T, LV2CompileError>;

// TODO: format output/display; only output first backtrace line that starts with `lovm2`
/// Runtime Error
#[derive(Debug)]
pub struct LV2Error {
    pub ty: LV2ErrorTy,
    pub msg: String,
    pub inx_offsets: Vec<usize>,

    #[cfg(feature = "backtrace")]
    pub trace: backtrace::Backtrace,
}

impl From<LV2ErrorTy> for LV2Error {
    fn from(ty: LV2ErrorTy) -> Self {
        Self {
            ty,
            ..Self::default()
        }
    }
}

impl From<std::io::Error> for LV2Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            ty: LV2ErrorTy::Custom(format!("{}", e)),
            ..Self::default()
        }
    }
}

impl<T> From<(LV2ErrorTy, T)> for LV2Error
where
    T: AsRef<str>,
{
    fn from(descr: (LV2ErrorTy, T)) -> Self {
        Self {
            ty: descr.0,
            msg: descr.1.as_ref().to_string(),
            ..Self::default()
        }
    }
}

impl From<(String, String)> for LV2Error {
    fn from(f: (String, String)) -> Self {
        Self {
            ty: LV2ErrorTy::Custom(f.0),
            msg: f.1,
            ..Self::default()
        }
    }
}

impl From<String> for LV2Error {
    fn from(msg: String) -> Self {
        Self {
            ty: LV2ErrorTy::Basic,
            msg,
            ..Self::default()
        }
    }
}

impl std::fmt::Display for LV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.ty, self.msg)?;

        if !self.inx_offsets.is_empty() {
            writeln!(f, "Instruction trace (deepest first):")?;
            for offset in self.inx_offsets.iter() {
                writeln!(f, "\t{}", offset)?;
            }
        }

        #[cfg(feature = "backtrace")]
        writeln!(f, "{:?}", self.trace)?;

        Ok(())
    }
}

impl Default for LV2Error {
    fn default() -> Self {
        Self {
            ty: LV2ErrorTy::Basic,
            msg: String::new(),
            inx_offsets: vec![],

            #[cfg(feature = "backtrace")]
            trace: backtrace::Backtrace::new(),
        }
    }
}
