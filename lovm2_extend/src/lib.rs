pub mod prelude;

pub const BASIC: u8 = 1;
pub const FRAME_STACK_EMPTY: u8 = 2;
pub const IMPORT_CONFLICT: u8 = 3;
pub const KEY_NOT_FOUND: u8 = 4;
pub const LOOKUP_FAILED: u8 = 5;
pub const MODULE_NOT_FOUND: u8 = 6;
pub const OPERATION_NOT_SUPPORTED: u8 = 7;
pub const VALUE_STACK_EMPTY: u8 = 8;

#[repr(C)]
pub struct Lovm2CError {
    pub ty: u8,
}

impl From<u8> for Lovm2CError {
    fn from(ty: u8) -> Self {
        Self { ty }
    }
}
