use lovm2_extend::prelude::*;

#[lovm2_object]
pub struct Buffer {
    inner: Vec<u8>,
}

/*
#[lovm2_function]
fn read(buffer: &mut Buffer, n: i64) -> Lovm2Result<String> {
    todo!()
}

#[lovm2_function]
fn read_line(buffer: &mut Buffer) -> Lovm2Result<String> {
    todo!()
}

#[lovm2_function]
fn write(buffer: &mut Buffer, text: String) -> Lovm2Result<bool> {
    todo!()
}
*/

lovm2_module_init!(buffer);
