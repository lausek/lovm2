use super::*;

use crate::value::LV2Value;

/// The operation is not supported.
#[inline]
pub fn err_not_supported<T>() -> LV2Result<T> {
    Err(LV2ErrorTy::OperationNotSupported.into())
}

/// The operation is not supported for this value type.
#[inline]
pub fn err_method_not_supported<T>(method: &str) -> LV2Result<T> {
    Err((LV2ErrorTy::OperationNotSupported, method).into())
}

/// The interrupt cannot be assigned from user code.
#[inline]
pub fn err_reserved_interrupt<T>(n: u16) -> LV2Result<T> {
    err_from_string(format!("interrupt {} is reserved", n))
}

/// Set instructions operand is not a [LV2Value::Ref].
#[inline]
pub fn err_invalid_set_target<T, U: ToString>(target: U) -> LV2Result<T> {
    Err((LV2ErrorTy::InvalidSetTarget, target.to_string()).into())
}

/// The interrupt cannot be assigned from user code.
#[inline]
pub fn err_key_not_found<T>(obj: &LV2Value, key: &LV2Value) -> LV2Result<T> {
    let msg = format!("key {} not found on value {}", key, obj);
    Err((LV2ErrorTy::KeyNotFound, msg).into())
}

/// Tried to operate on an empty reference.
#[inline]
pub fn err_empty_dereference<T>() -> LV2Result<T> {
    Err(LV2Error {
        msg: "dereference on empty".to_string(),
        ..LV2Error::default()
    })
}

/// Unexpected type, expected other type instead.
#[inline]
pub fn err_ty_unexpected<U: std::fmt::Display, V: std::fmt::Display>(
    expected: U,
    got: V,
) -> LV2Error {
    LV2Error {
        msg: format!("expected {}, got {}", expected, got),
        ..LV2Error::default()
    }
}

/// Create an error from a custom message.
#[inline]
pub fn err_custom<T: std::fmt::Display>(msg: T) -> LV2Error {
    LV2Error {
        msg: format!("{}", msg),
        ..LV2Error::default()
    }
}

/// Shared object symbol was not found.
#[inline]
pub fn err_symbol_not_found<T, U: std::fmt::Display>(name: U) -> LV2Result<T> {
    err_from_string(format!(
        "symbol `{}` cannot be loaded from shared object",
        name
    ))
}

/// Create a custom error from string.
#[inline]
pub fn err_from_string<T, U: ToString>(msg: U) -> LV2Result<T> {
    Err(LV2Error {
        msg: msg.to_string(),
        ..LV2Error::default()
    })
}

/// `next` was called on an iterator even though it is empty.
#[inline]
pub fn err_iterator_exhausted<T>() -> LV2Result<T> {
    err_from_string(format!("iterator exhausted"))
}
