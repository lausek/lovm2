use lovm2::prelude::*;
use lovm2_extend::prelude::*;

#[lovm2_object]
pub struct File {
    inner: std::fs::File,
}

#[lovm2_function]
fn create_file(path: String) -> Lovm2Result<File> {
    std::fs::File::create(path)
        .map(|inner| File { inner })
        .map_err(|e| e.to_string().into())
}

#[lovm2_function]
fn open_file(path: String) -> Lovm2Result<File> {
    std::fs::File::open(path)
        .map(|inner| File { inner })
        .map_err(|e| e.to_string().into())
}

#[lovm2_function]
fn read_all(file: &mut File) -> Lovm2Result<String> {
    use std::io::Read;
    let mut buffer = String::new();
    file.inner
        .read_to_string(&mut buffer)
        .map_err(|e| Lovm2Error::from(e.to_string()))?;
    Ok(buffer)
}

#[lovm2_function]
fn write_all(file: &mut File, content: String) -> Lovm2Result<bool> {
    use std::io::Write;
    file.inner
        .write_all(content.as_bytes())
        .map_err(|e| Lovm2Error::from(e.to_string()))?;
    Ok(true)
}

#[lovm2_function]
fn absolute(path: String) -> Lovm2Result<String> {
    std::fs::canonicalize(path)
        .map(|buf| buf.to_string_lossy().into_owned())
        .map_err(|e| Lovm2Error::from(e.to_string()))
}

#[lovm2_function]
fn basename(path: String) -> Option<String> {
    std::path::Path::new(&path)
        .file_name()
        .map(|buf| buf.to_string_lossy().into_owned())
}

#[lovm2_function]
fn parent(path: String) -> Option<String> {
    std::path::Path::new(&path)
        .parent()
        .map(|buf| buf.to_string_lossy().into_owned())
}

#[lovm2_function]
fn exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}

#[lovm2_function]
fn mkdir(path: String) -> bool {
    std::fs::create_dir_all(path).is_ok()
}

#[lovm2_function]
fn rmdir(path: String) -> bool {
    std::fs::remove_dir(path).is_ok()
}

#[lovm2_function]
fn is_dir(path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}

#[lovm2_function]
fn list_dir(path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}

#[lovm2_function]
fn unlink(path: String) -> bool {
    std::fs::remove_file(path).is_ok()
}

lovm2_module_init!();
