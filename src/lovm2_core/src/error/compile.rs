/// Compiletime error.
#[derive(Debug)]
pub struct LV2CompileError {
    pub ty: Option<String>,
    pub msg: String,
}

impl<T> From<T> for LV2CompileError
where
    T: ToString,
{
    fn from(msg: T) -> Self {
        Self {
            ty: None,
            msg: msg.to_string(),
        }
    }
}
