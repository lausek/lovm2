#![cfg(test)]

use test_utils::*;

use tempfile::tempdir;

use lovm2_core::extend::prelude::*;

#[test]
fn file_creation_deletion() {
    let dir = tempdir().unwrap();

    let mut vm = run_module_test(|_| {});

    let path = dir.path().join(".file");
    let path_str = path.to_str().unwrap();

    vm.call("create_file", &[path_str.into()]).unwrap();

    assert!(path.exists());

    vm.call("unlink", &[path_str.into()]).unwrap();

    assert!(!path.exists());
}

#[test]
fn file_rename() {
    let dir = tempdir().unwrap();

    let mut vm = run_module_test(|_| {});

    let path = dir.path().join(".file");
    let path_str = path.to_str().unwrap();
    let other = dir.path().join(".other");
    let other_str = other.to_str().unwrap();

    vm.call("create_file", &[path_str.into()]).unwrap();

    assert!(path.exists());
    assert!(!other.exists());

    vm.call("rename", &[path_str.into(), other_str.into()])
        .unwrap();

    assert!(!path.exists());
    assert!(other.exists());
}

#[test]
fn file_list_dir() {
    let dir = tempdir().unwrap();

    let mut vm = run_module_test(|_| {});

    let root = dir.path().join("root").join("next");
    let root_str = root.to_str().unwrap();

    assert!(!root.exists());
    assert_eq!(
        Value::from(false),
        vm.call("exists", &[root_str.into()]).unwrap(),
    );

    vm.call("mkdir", &[root_str.into()]).unwrap();

    assert!(root.exists() && root.is_dir());
    assert_eq!(
        Value::from(true),
        vm.call("exists", &[root_str.into()]).unwrap(),
    );
    assert_eq!(
        Value::from(true),
        vm.call("is_dir", &[root_str.into()]).unwrap(),
    );

    assert_eq!(
        0,
        vm.call("list_dir", &[root_str.into()])
            .unwrap()
            .len()
            .unwrap()
    );

    for fname in ["a", "b"].iter() {
        let root_file = root.join(fname);
        vm.call("create_file", &[root_file.to_str().unwrap().into()])
            .unwrap();
        assert!(root_file.exists() && root_file.is_file());
    }

    let dir_files = vm.call("list_dir", &[root_str.into()]).unwrap();

    assert_eq!(2, dir_files.len().unwrap());

    // cannot unlink directory
    assert_eq!(
        Value::from(false),
        vm.call("unlink", &[root_str.into()]).unwrap()
    );

    // directory is not empty
    assert_eq!(
        Value::from(false),
        vm.call("rmdir", &[root_str.into()]).unwrap(),
    );

    assert!(root.exists() && root.is_dir());

    let mut it = dir_files.iter().unwrap();
    while it.has_next() {
        let dir_file = it.next().unwrap();
        assert_eq!(
            Value::from(true),
            vm.call("unlink", &[dir_file.into()]).unwrap()
        );
    }

    // directory was deleted
    assert_eq!(
        Value::from(true),
        vm.call("rmdir", &[root_str.into()]).unwrap(),
    );

    assert!(!root.exists());
}

#[test]
fn filepath_operations() {
    let dir = tempdir().unwrap();

    let mut vm = run_module_test(|_| {});

    let root = dir.path().join("a").join("b");
    let root_str = root.to_str().unwrap();
    let parent = vm.call("parent", &[root_str.into()]).unwrap();

    assert_eq!(
        Value::from("b"),
        vm.call("basename", &[root_str.into()]).unwrap(),
    );
    assert_eq!(Value::from("a"), vm.call("basename", &[parent]).unwrap(),);
}
