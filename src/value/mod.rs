//! Representation and operations for lovm2 values

use indexmap::IndexMap;
use lovm2_error::*;

mod cast;
mod data;
mod iter;
mod op;
mod opi;
mod r#ref;

pub use self::cast::*;
pub use self::data::{box_value, AnyRef, Value, ValueRef};
pub use self::r#ref::Reference;

pub(crate) use self::iter::register_iter_interrupts;
