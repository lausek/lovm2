use lovm2::value::box_value;
use lovm2_extend::prelude::*;

#[lovm2_object]
pub struct File {
    inner: std::fs::File,
}

#[lovm2_function]
fn new_file(path: String) -> File {
    todo!()
}

#[lovm2_function]
fn open_file(path: String) -> File {
    todo!()
}

#[lovm2_function]
fn read_all(file: &File) -> Lovm2Result<String> {
    todo!()
}

#[lovm2_function]
fn write_all(file: &mut File, content: String) -> Lovm2Result<bool> {
    todo!()
}

#[lovm2_function]
fn absolute(path: String) -> String {
    todo!()
}

#[lovm2_function]
fn basename(path: String) -> String {
    todo!()
}

#[lovm2_function]
fn exists(path: String) -> bool {
    todo!()
}

#[lovm2_function]
fn mkdir(path: String) -> bool {
    todo!()
}

#[lovm2_function]
fn rmdir(path: String) -> bool {
    todo!()
}

#[lovm2_function]
fn is_dir(path: String) -> bool {
    todo!()
}

#[lovm2_function]
fn unlink(path: String) -> bool {
    todo!()
}

lovm2_module_init!();
