//! representation and operations for lovm2 values

pub mod cast;
pub mod data;
pub mod op;
pub mod opi;

pub use self::data::{box_value, Value, ValueRef};

fn not_supported<T>() -> lovm2_error::Lovm2Result<T> {
    Err(lovm2_error::Lovm2ErrorTy::OperationNotSupported.into())
}
