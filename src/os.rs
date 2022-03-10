use core::{panic::PanicInfo, fmt::{Write, self}, ffi::c_void};

use crate::Error;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
mod linux;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
use linux as inner;

#[cfg(all(target_os="macos", target_arch="x86_64"))]
mod macos;
#[cfg(all(target_os="macos", target_arch="x86_64"))]
use macos as inner;

#[no_mangle]
fn __libc_csu_fini() {}

#[no_mangle]
fn __libc_csu_init() {}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn __libc_start_main(
    main: extern "C" fn(isize, *const *const u8) -> u8,
    argc: isize,
    argv: *const *const u8,
    _csu_init: extern "C" fn(*mut c_void),
    _csu_fini: extern "C" fn(*mut c_void),
    _rtld_fini: extern "C" fn(*mut c_void),
    _stack_end: *mut c_void,
) {
    let ret = main(argc, argv);
    inner::exit(ret);
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> u8 {
    if let Err(_) = crate::main() {
        let _ = writeln!(STDERR, "Error!");
        1
    } else {
        0
    }
}

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
    inner::exit(1);
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

pub fn map_file() {
    unimplemented!()
}

pub fn open_for_log() {
    unimplemented!()
}

pub fn open_for_read(path: impl AsRef<[u8]>) -> Result<Fd, Error> {
    unimplemented!()
}

pub fn write(fd: Fd, msg: impl AsRef<[u8]>) -> Result<(), Error> {
    unimplemented!()
}