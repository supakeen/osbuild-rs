use super::*;
use cap_std::ambient_authority;
use cap_std::fs::Dir;
use std::ffi::OsStr;

#[test]
fn test_get() {
    let root = Dir::open_ambient_dir("/", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("etc/passwd").expect("Failed to open file.");

    let label = &OsStr::new("security.selinux");
    let value = "system_u:object_r:passwd_file_t:s0\0".as_bytes().to_vec();

    assert!(file.xattr_get(label).unwrap().is_some());

    assert_eq!(file.xattr_get(label).unwrap(), Some(value),);
}

#[test]
fn test_set() {
    let root = Dir::open_ambient_dir("/", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("etc/passwd").expect("Failed to open file.");

    let label = &OsStr::new("security.selinux");
    let value = "system_u:object_r:passwd_file_t:s0\0".as_bytes().to_vec();

    assert!(file.xattr_get(label).unwrap().is_some());

    assert_eq!(file.xattr_get(label).unwrap(), Some(value),);
}

#[test]
fn test_remove() {
    let root =
        Dir::open_ambient_dir("/tmp", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("test_remove").expect("Failed to open file.");

    assert!(file
        .xattr_get(&OsStr::new("security.selinux"))
        .unwrap()
        .is_some());

    assert_eq!(
        file.xattr_get(&OsStr::new("security.selinux")).unwrap(),
        Some("system_u:object_r:tmp_t:s0\0".as_bytes().to_vec())
    );

    assert!(file.xattr_remove(&OsStr::new("security.selinux")).is_ok());

    assert!(file
        .xattr_get(&OsStr::new("security.selinux"))
        .unwrap()
        .is_none());
}

#[test]
fn test_list() {
    let root = Dir::open_ambient_dir("/", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("etc/passwd").expect("Failed to open file.");

    assert!(file.xattr_list().is_ok());
}
