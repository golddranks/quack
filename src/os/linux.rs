use core::{arch::asm, ffi::c_void, fmt::{self, Write}};

// Items required by no_std

#[no_mangle]
fn __libc_csu_fini() {}

#[no_mangle]
fn __libc_csu_init() {}

#[no_mangle]
extern "C" fn __libc_start_main(
    main: extern "C" fn(isize, *const *const u8) -> isize,
    argc: isize,
    argv: *const *const u8,
    _csu_init: extern "C" fn(*mut c_void),
    _csu_fini: extern "C" fn(*mut c_void),
    _rtld_fini: extern "C" fn(*mut c_void),
    _stack_end: *mut c_void,
) {
    let ret = main(argc, argv);
    exit(ret);
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    if let Err(_) = crate::main() {
        let _ = writeln!(STDERR, "Error!");
        -1
    } else {
        0
    }
}


// Syscall implementations

#[repr(u64)]
enum Syscall {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Mmap = 9,
    Exit = 60,
}

pub fn exit(ret: isize) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Exit as i64,
            in("rdi") ret,
            out("rcx") _,
            out("r11") _,
        );
    }
    loop {}
}

pub const STDIN: Fd = Fd(0);
pub const STDOUT: Fd = Fd(1);
pub const STDERR: Fd = Fd(2);

pub struct WriteError(isize);

impl From<WriteError> for fmt::Error {
    fn from(_: WriteError) -> Self {
        Self
    }
}

impl From<WriteError> for crate::Error {
    fn from(_: WriteError) -> Self {
        Self
    }
}

pub fn write(fd: Fd, msg: impl AsRef<[u8]>) -> Result<usize, WriteError> {
    let msg = msg.as_ref();
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Write as i64,
            in("rdi") fd.0,
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    if ret >= 0 {
        Ok(ret as usize)
    } else {
        Err(WriteError(ret))
    }
}

pub fn read(fd: Fd, buf: &mut [u8]) -> Result<usize, WriteError> {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Read as i64,
            in("rdi") fd.0,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    if ret >= 0 {
        Ok(ret as usize)
    } else {
        Err(WriteError(ret)) // TODO: ReadError
    }
}

pub struct OpenError(isize);

impl From<OpenError> for crate::Error {
    fn from(_: OpenError) -> Self {
        Self
    }
}

#[repr(i64)]
#[derive(Copy, Clone)]
pub enum OpenMode {
    RdOnly = 0x000,
    WrOnly = 0x001,
    RdWr = 0x002,
    Creat = 0x100,
    Append = 0x2000,
}


pub fn open(path: impl AsRef<[u8]>, mode: OpenMode) -> Result<Fd, OpenError> {
    let path = path.as_ref();
    if let Some(b'\0') = path.last() {} else {
        panic!("path must be null-terminated");
    };
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Open as i64,
            in("rdi") path.as_ref().as_ptr(),
            in("rsi") mode as i64,
            in("rdx") 0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    if ret < 0 {
        Err(OpenError(ret))
    } else {
        Ok(Fd(ret))
    }
}