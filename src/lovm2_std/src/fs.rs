use super::*;

#[lovm2_function]
fn create_file(path: String) -> LV2Result<File> {
    std::fs::File::create(path)
        .map(|inner| File { inner })
        .map_err(LV2Error::from)
}

#[lovm2_function]
fn open_file(path: String) -> LV2Result<File> {
    std::fs::File::open(path)
        .map(|inner| File { inner })
        .map_err(LV2Error::from)
}

#[lovm2_function]
fn read_all(file: &mut File) -> LV2Result<String> {
    use std::io::Read;

    let mut buffer = String::new();

    file.inner
        .read_to_string(&mut buffer)
        .map_err(LV2Error::from)?;

    Ok(buffer)
}

#[lovm2_function]
fn write_all(file: &mut File, content: String) -> LV2Result<bool> {
    use std::io::Write;

    file.inner
        .write_all(content.as_bytes())
        .map_err(LV2Error::from)?;

    Ok(true)
}

#[lovm2_function]
fn absolute(path: String) -> LV2Result<String> {
    std::fs::canonicalize(path)
        .map(|buf| buf.to_string_lossy().into_owned())
        .map_err(LV2Error::from)
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
fn list_dir(path: String) -> LV2Result<LV2Value> {
    let mut entries = vec![];

    for entry in std::fs::read_dir(path).map_err(LV2Error::from)? {
        let entry = entry?;

        if let Some(entry) = entry.path().to_str() {
            entries.push(LV2Value::from(entry));
        } else {
            err_from_string("could not read dir entry")?
        }
    }

    Ok(box_value(LV2Value::List(entries)))
}

#[lovm2_function]
fn unlink(path: String) -> bool {
    std::fs::remove_file(path).is_ok()
}

#[lovm2_function]
fn rename(from: String, to: String) -> LV2Result<bool> {
    std::fs::rename(from, to).map_err(LV2Error::from)?;

    Ok(true)
}
