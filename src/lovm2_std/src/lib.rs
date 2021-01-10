mod buffer;
mod collection;
mod data;
mod fs;
mod json;
mod math;
#[cfg(feature = "net")]
mod net;
mod regex;
mod string;

pub use self::buffer::*;
pub use self::collection::*;
pub use self::data::*;
pub use self::fs::*;
pub use self::json::*;
pub use self::math::*;
#[cfg(feature = "net")]
pub use self::net::*;
pub use self::regex::*;
pub use self::string::*;

pub fn create_std_module() -> lovm2_extend::prelude::Module {
    todo!()
}
