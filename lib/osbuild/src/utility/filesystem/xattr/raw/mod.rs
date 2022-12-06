// This code comes from: https://github.com/Stebalien/xattr and copyrights lay there. I've chosen
// to copy most of this implementation with small adjustments for our specific needs and
// `cap_std` compatibility which is our default avenue to access filesystems.

use libc::{
    c_char, c_int, c_void, fgetxattr, flistxattr, fremovexattr, fsetxattr, size_t, ssize_t, ERANGE,
};
use std::ffi::{CString, OsStr, OsString};
use std::io;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::BorrowedFd;
use std::ptr;

#[cfg(test)]
mod test;

#[allow(dead_code)]
fn name_to_c(name: &OsStr) -> io::Result<CString> {
    match CString::new(name.as_bytes()) {
        Ok(name) => Ok(name),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "name must not contain null bytes",
        )),
    }
}

pub unsafe fn allocate_loop<F: FnMut(*mut u8, usize) -> ssize_t>(mut f: F) -> io::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::new();
    loop {
        let ret = (f)(ptr::null_mut(), 0);
        if ret < 0 {
            return Err(io::Error::last_os_error());
        } else if ret == 0 {
            break;
        }
        vec.reserve_exact(ret as usize);

        let ret = (f)(vec.as_mut_ptr(), vec.capacity());
        if ret >= 0 {
            vec.set_len(ret as usize);
            break;
        } else {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(ERANGE) {
                continue;
            }
            return Err(error);
        }
    }
    vec.shrink_to_fit();
    Ok(vec)
}

pub struct XAttrs {
    data: Box<[u8]>,
    offset: usize,
}

impl Clone for XAttrs {
    fn clone(&self) -> Self {
        XAttrs {
            data: Vec::from(&*self.data).into_boxed_slice(),
            offset: self.offset,
        }
    }
    fn clone_from(&mut self, other: &XAttrs) {
        self.offset = other.offset;

        let mut data = mem::replace(&mut self.data, Box::new([])).into_vec();
        data.extend(other.data.iter().cloned());
        self.data = data.into_boxed_slice();
    }
}

impl Iterator for XAttrs {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        let data = &self.data[self.offset..];
        if data.is_empty() {
            None
        } else {
            // always null terminated (unless empty).
            let end = data.iter().position(|&b| b == 0u8).unwrap();
            self.offset += end + 1;
            Some(OsStr::from_bytes(&data[..end]).to_owned())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.data.len() == self.offset {
            (0, Some(0))
        } else {
            (1, None)
        }
    }
}

pub fn get(fd: BorrowedFd, name: &OsStr) -> io::Result<Vec<u8>> {
    let c_name = name_to_c(name)?;
    let c_fd = fd.as_raw_fd();

    unsafe {
        allocate_loop(|buf, len| {
            fgetxattr(
                c_fd as c_int,
                c_name.as_ptr(),
                buf as *mut c_void,
                len as size_t,
            )
        })
    }
}

pub fn set(fd: BorrowedFd, name: &OsStr, value: &[u8]) -> io::Result<()> {
    let c_name = name_to_c(name)?;
    let c_fd = fd.as_raw_fd();

    let ret = unsafe {
        fsetxattr(
            c_fd as c_int,
            c_name.as_ptr(),
            value.as_ptr() as *const c_void,
            value.len() as size_t,
            0 as c_int,
        )
    };

    if ret != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn remove(fd: BorrowedFd, name: &OsStr) -> io::Result<()> {
    let c_name = name_to_c(name)?;
    let c_fd = fd.as_raw_fd();

    let ret = unsafe { fremovexattr(c_fd as c_int, c_name.as_ptr()) };

    if ret != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn list(fd: BorrowedFd) -> io::Result<XAttrs> {
    let c_fd = fd.as_raw_fd();

    let vec = unsafe {
        allocate_loop(|buf, len| flistxattr(c_fd as c_int, buf as *mut c_char, len as size_t))?
    };

    Ok(XAttrs {
        data: vec.into_boxed_slice(),
        offset: 0,
    })
}
