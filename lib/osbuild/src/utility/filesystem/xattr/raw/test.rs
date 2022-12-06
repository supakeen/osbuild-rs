use cap_std::ambient_authority;
use cap_std::fs::Dir;
use cap_std::io_lifetimes::AsFd;
use std::ffi::OsStr;
use std::io::Write;
use std::str;

use super::*;

#[test]
fn test_get() {
    let root = Dir::open_ambient_dir("/", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("etc/passwd").expect("Failed to open file.");

    let value = get(file.as_fd(), &OsStr::new("security.selinux")).expect("Failed to call `get`.");

    assert_eq!(
        str::from_utf8(&value).unwrap(),
        "system_u:object_r:passwd_file_t:s0\0"
    );
}

#[test]
fn test_set() {
    let root =
        Dir::open_ambient_dir("/tmp", ambient_authority()).expect("Failed to open root dir.");

    if root.exists("fakename") {
        root.remove_file("fakename").unwrap();
    }

    let mut file = root.create("fakename").expect("Failed to open file.");
    file.write("testcase".as_bytes())
        .expect("Failed to write to file.");

    let label = &OsStr::new("user.testcase");
    let value = "value".as_bytes();

    set(file.as_fd(), label, value).expect("Failed to call `set`.");

    file.sync_all().expect("Failed to sync file.");

    let value = get(file.as_fd(), label).expect("Failed to call `get`.");

    assert_eq!(str::from_utf8(&value).unwrap().as_bytes(), value,);
}

#[test]
fn test_remove() {
    let root = Dir::open_ambient_dir("/", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("etc/passwd").expect("Failed to open file.");

    let value = get(file.as_fd(), &OsStr::new("security.selinux")).expect("Failed to call `get`.");

    println!("{:?}", value);

    assert_eq!(
        str::from_utf8(&value).unwrap(),
        "system_u:object_r:passwd_file_t:s0\0"
    );
}

#[test]
fn test_list() {
    let root = Dir::open_ambient_dir("/", ambient_authority()).expect("Failed to open root dir.");
    let file = root.open("etc/passwd").expect("Failed to open file.");

    let value = get(file.as_fd(), &OsStr::new("security.selinux")).expect("Failed to call `get`.");

    println!("{:?}", value);

    assert_eq!(
        str::from_utf8(&value).unwrap(),
        "system_u:object_r:passwd_file_t:s0\0"
    );
}
