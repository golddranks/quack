use core::{
    fmt::{self, Write},
    panic::PanicInfo, slice,
};

use crate::Error;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use linux as inner;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod macos;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use macos as inner;

#[derive(Copy, Clone)]
pub struct Fd(u32);

pub const STDERR: Fd = Fd(2);

impl Write for Fd {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        write(*self, s)?;
        Ok(())
    }
}

#[panic_handler]
fn panic(pi: &PanicInfo) -> ! {
    if let Some(loc) = pi.location() {
        let _ = writeln!(STDERR, "panic: {:?}", loc);
    } else {
        let _ = writeln!(STDERR, "panic");
    }
    inner::exit(2);
}

#[no_mangle]
pub unsafe extern "C" fn memset(mut s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let end = s.add(n);
    while s < end {
        *s = c as u8;
        s = s.add(1);
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(mut dst: *mut u8, mut src: *const u8, count: usize) -> *mut u8 {
    let end = src.add(count);
    while src < end {
        *dst = *src;
        src = src.add(1);
        dst = dst.add(1);
    }
    dst
}

pub struct MappedFile(&'static mut [u8]);

impl MappedFile {
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}

pub struct Args(&'static [*const u8]);

impl Args {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn nth(&self, n: usize) -> &[u8] {
        let base = self.0[n];
        let mut ptr = base;
        loop {
            let c = unsafe { *ptr };
            if c == b'\0' {
                break;
            }
            ptr = unsafe { ptr.offset(1) };
        }
        unsafe { slice::from_raw_parts(base,  ptr.offset_from(base) as usize) }
    }
}

pub fn map_file(_fd: Fd) -> Result<MappedFile, Error> {
    unimplemented!()
}

pub fn open_for_log(_path: impl AsRef<[u8]>) -> Result<Fd, Error> {
    unimplemented!()
}

pub fn open_for_read(_path: impl AsRef<[u8]>) -> Result<Fd, Error> {
    unimplemented!()
}

pub fn write(fd: Fd, msg: impl AsRef<[u8]>) -> Result<usize, Error> {
    inner::write(fd, msg)
}
