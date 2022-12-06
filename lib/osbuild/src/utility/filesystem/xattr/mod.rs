pub mod raw;

#[cfg(test)]
mod test;

use cap_std::fs::File;
use libc::ENODATA;
use std::ffi::OsStr;
use std::io;
use std::os::unix::io::AsFd;

fn enodata_to_none(result: io::Result<Vec<u8>>) -> io::Result<Option<Vec<u8>>> {
    result.map(Some).or_else(|e| match e.raw_os_error() {
        Some(ENODATA) => Ok(None),
        _ => Err(e),
    })
}

pub trait FileExt: AsFd {
    fn xattr_get(&self, name: impl AsRef<OsStr>) -> io::Result<Option<Vec<u8>>> {
        enodata_to_none(raw::get(self.as_fd(), name.as_ref()))
    }

    /// Set an extended attribute on the specified file.
    fn xattr_set(&self, name: impl AsRef<OsStr>, value: &[u8]) -> io::Result<()> {
        raw::set(self.as_fd(), name.as_ref(), value)
    }

    /// Remove an extended attribute from the specified file.
    fn xattr_remove(&self, name: impl AsRef<OsStr>) -> io::Result<()> {
        raw::remove(self.as_fd(), name.as_ref())
    }

    /// List extended attributes attached to the specified file.
    ///
    /// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
    /// attributes unless you are root and it may not list system attributes.
    fn xattr_list(&self) -> io::Result<raw::XAttrs> {
        raw::list(self.as_fd())
    }

    /// Copy all attributes as found by `xattr_list` and create/set them on the other File.
    fn xattr_copy(&self, _other: File) -> io::Result<()> {
        Ok(())
    }
}

impl FileExt for File {}
