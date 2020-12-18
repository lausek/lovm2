//! Representation and operations for lovm2 values

mod cast;
mod data;
mod op;
mod opi;

pub use self::cast::*;
pub use self::data::{box_value, AnyRef, Value, ValueRef};

#[inline]
fn not_supported<T>() -> lovm2_error::Lovm2Result<T> {
    Err(lovm2_error::Lovm2ErrorTy::OperationNotSupported.into())
}
