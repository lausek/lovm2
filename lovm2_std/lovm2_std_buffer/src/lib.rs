use lovm2_extend::prelude::*;
use lovm2_std_data::*;

#[inline]
fn inner_readn(buffer: &mut Buffer, n: usize) -> Lovm2Result<String> {
    let upto = buffer.roff + n;
    let upto = std::cmp::min(upto, buffer.inner.len());
    let s = String::from_utf8_lossy(&buffer.inner[buffer.roff..upto]).to_string();
    buffer.roff = upto;
    Ok(s)
}

#[lovm2_function]
fn new_buffer() -> Lovm2Result<Buffer> {
    Ok(Buffer::default())
}

#[lovm2_function]
fn readn(buffer: &mut Buffer, n: i64) -> Lovm2Result<String> {
    inner_readn(buffer, n as usize)
}

#[lovm2_function]
fn read_line(buffer: &mut Buffer) -> Lovm2Result<String> {
    let mut n = 0;

    while let Some(c) = buffer.inner.get(n) {
        n += 1;
        if *c == '\n' as u8 {
            break;
        }
    }

    inner_readn(buffer, n)
}

#[lovm2_function]
fn writes(buffer: &mut Buffer, text: String) -> Lovm2Result<bool> {
    buffer.inner.extend_from_slice(text.as_bytes());
    Ok(true)
}

#[lovm2_function]
fn has_data(buffer: &mut Buffer) -> bool {
    buffer.inner.len() != buffer.roff
}

lovm2_module_init!(buffer);
