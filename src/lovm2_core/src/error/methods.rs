use super::*;

/// The operation is not supported
#[inline]
pub fn err_not_supported<T>() -> Lovm2Result<T> {
    Err(Lovm2ErrorTy::OperationNotSupported.into())
}

/// The operation is not supported for this value type
#[inline]
pub fn err_method_not_supported<T>(method: &str) -> Lovm2Result<T> {
    Err((Lovm2ErrorTy::OperationNotSupported, method).into())
}

/// The interrupt cannot be assigned from user code
#[inline]
pub fn err_reserved_interrupt<T>(n: u16) -> Lovm2Result<T> {
    err_from_string(format!("interrupt {} is reserved", n))
}

/// Set instructions operand is not a `Ref`
#[inline]
pub fn err_invalid_set_target<T, U: ToString>(target: U) -> Lovm2Result<T> {
    Err((Lovm2ErrorTy::InvalidSetTarget, target.to_string()).into())
}

#[inline]
pub fn err_empty_dereference<T>() -> Lovm2Result<T> {
    Err(Lovm2Error {
        msg: "dereference on empty".to_string(),
        ..Lovm2Error::default()
    })
}

/// Unexpected type, expected other type instead
#[inline]
pub fn err_ty_unexpected<U: std::fmt::Display, V: std::fmt::Display>(expected: U, got: V) -> Lovm2Error {
    Lovm2Error {
        msg: format!("expected {}, got {}", expected, got),
        ..Lovm2Error::default()
    }
}

#[inline]
pub fn err_custom<T: std::fmt::Display>(msg: T) -> Lovm2Error {
    Lovm2Error {
        msg: format!("{}", msg),
        ..Lovm2Error::default()
    }
}

/// Shared object symbol was not found
#[inline]
pub fn err_symbol_not_found<T, U: std::fmt::Display>(name: U) -> Lovm2Result<T> {
    err_from_string(format!("symbol `{}` cannot be loaded from shared object", name))
}

/// Create a custom error from string
#[inline]
pub fn err_from_string<T, U: ToString>(msg: U) -> Lovm2Result<T>
{
    Err(Lovm2Error {
        msg: msg.to_string(),
        ..Lovm2Error::default()
    })
}

/// `next` was called on an iterator even though it is empty
#[inline]
pub fn err_iterator_exhausted<T>() -> Lovm2Result<T> {
    err_from_string(format!("iterator exhausted"))
}
