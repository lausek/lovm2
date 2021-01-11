use super::*;

#[inline]
pub fn err_not_supported<T>() -> Lovm2Result<T> {
    Err(Lovm2ErrorTy::OperationNotSupported.into())
}

#[inline]
pub fn err_method_not_supported<T>(method: &str) -> Lovm2Result<T> {
    Err((Lovm2ErrorTy::OperationNotSupported, method).into())
}

#[inline]
pub fn err_reserved_interrupt<T>(n: u16) -> Lovm2Result<T> {
    err_from_string(format!("interrupt {} is reserved", n))
}

#[inline]
pub fn err_from_string<U, T>(msg: T) -> Lovm2Result<U>
where T: ToString
{
    Err(Lovm2Error {
        msg: msg.to_string(),
        ..Lovm2Error::default()
    })
}

#[inline]
pub fn err_iterator_exhausted<T>() -> Lovm2Result<T> {
    err_from_string(format!("iterator exhausted"))
}
