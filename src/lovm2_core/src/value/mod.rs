//! Representation and operations for lovm2 values

use indexmap::IndexMap;

use crate::error::*;

mod conv;
mod data;
mod op;
mod opi;
mod r#ref;

pub(crate) mod iter;

pub use self::conv::*;
pub use self::data::{box_value, AnyRef, Value, ValueRef};
pub use self::iter::Iter;
pub use self::r#ref::Reference;
