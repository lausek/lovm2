//! Representation and operations for lovm2 values

use lovm2_error::*;

mod cast;
mod data;
mod op;
mod opi;
mod r#ref;

pub use self::cast::*;
pub use self::data::{box_value, AnyRef, Value, ValueRef};
pub use self::r#ref::Reference;

#[inline]
fn not_supported<T>() -> lovm2_error::Lovm2Result<T> {
    Err(lovm2_error::Lovm2ErrorTy::OperationNotSupported.into())
}
