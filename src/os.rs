use core::{
    fmt::{self, Write},
    panic::PanicInfo, slice, ptr::null,
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

#[no_mangle]
#[allow(unused_unsafe)]
unsafe extern "C" fn start2(argc: i64, argv: *const *const u8) -> ! {
    let args: &[*const u8] = unsafe { slice::from_raw_parts(argv, argc as usize) };
    if let Err(e) = crate::main(Args(args)) {
        let _ = writeln!(crate::os::STDERR, "Stopped because of {:?} error.", e);
        inner::exit(e.to_ret())
    } else {
        inner::exit(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Fd(u32);

pub const STDERR: Fd = Fd(2);

impl Write for Fd {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        write(*self, s)?;
        Ok(())
    }
}

#[cfg(not(test))]
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

#[no_mangle]
pub unsafe extern "C" fn memcmp(mut s1: *const u8, mut s2: *const u8, count: usize) -> i32 {
    let end = s2.add(count);
    while s2 < end {
        let v1 = *s1 as i32;
        let v2 = *s2 as i32;
        let diff = v1 - v2;
        if diff != 0 {
            return diff as i32;
        }
        s2 = s2.add(1);
        s1 = s1.add(1);
    }
    0
}

#[derive(Debug)]
pub enum MappedFile {
    ReadWrite(&'static mut [u8]),
    ReadOnly(&'static [u8]),
}

impl MappedFile {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::ReadOnly(m) => m,
            Self::ReadWrite(m) => m,
        }
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
            ptr = unsafe { ptr.offset(1) };
            if c == b'\0' {
                break;
            }
        }
        unsafe { slice::from_raw_parts(base,  ptr.offset_from(base) as usize) }
    }
}

pub fn map_file(fd: Fd) -> Result<MappedFile, Error> {
    let stat = inner::fstat(fd)?;
    inner::mmap(
        null(),
        stat.size,
        inner::mmap_prot::PROT_READ | inner::mmap_prot::PROT_WRITE,
        inner::mmap_flags::MAP_PRIVATE,
        fd,
        0)
}

pub fn open_for_log(path: impl AsRef<[u8]>) -> Result<Fd, Error> {
    inner::open(path.as_ref(),
    inner::OpenMode::CREAT | inner::OpenMode::WR_ONLY | inner::OpenMode::APPEND,
    0b110100100) // 0644
}

pub fn open_for_read(path: impl AsRef<[u8]>) -> Result<Fd, Error> {
    inner::open(path.as_ref(), inner::OpenMode::RD_ONLY, 0)
}

pub fn write(fd: Fd, msg: impl AsRef<[u8]>) -> Result<usize, Error> {
    inner::write(fd, msg)
}
