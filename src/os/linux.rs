use core::{
    arch::{asm, global_asm},
    ffi::c_void,
    fmt::Write,
    slice,
};

use crate::{os::Fd, Error};

global_asm!(
    ".globl _start
_start: mov    rdi, rsp # pass pointer to argc to start2; rdi is used for the first arg
        and    rsp, 0xfffffffffffffff0 # align stack to 16 bytes; expected by x86-64 Linux C ABI
        call   start2"
);

#[no_mangle]
unsafe extern "C" fn start2(stack_start: *const c_void) -> ! {
    let argc = unsafe { *(stack_start as *const usize) };
    let argv: *const *const u8 = unsafe { (stack_start as *const *const u8).offset(1) };
    let args: &[*const u8] = unsafe { slice::from_raw_parts(argv, argc) };
    if let Err(_) = crate::main(args) {
        let _ = write(crate::os::STDERR, "Error!\n");
        exit(1)
    } else {
        exit(0)
    }
}

#[repr(u32)]
enum Syscall {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Mmap = 9,
    Exit = 60,
}

pub fn exit(ret: u8) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Exit as u32,
            in("rdi") ret as i64,
            out("rcx") _,
            out("r11") _,
        );
    }
    loop {}
}

pub fn write(fd: Fd, msg: impl AsRef<[u8]>) -> Result<usize, Error> {
    let msg = msg.as_ref();
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Write as u32,
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
        Err(Error::Write(ret as i32))
    }
}

pub fn read(fd: Fd, buf: &mut [u8]) -> Result<usize, Error> {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Read as u32,
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
        Err(Error::Read(ret as i32))
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

pub fn open(path: impl AsRef<[u8]>, mode: OpenMode) -> Result<Fd, Error> {
    let path = path.as_ref();
    if let Some(b'\0') = path.last() {
    } else {
        panic!("path must be null-terminated");
    };
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Open as u32,
            in("rdi") path.as_ref().as_ptr(),
            in("rsi") mode as i64,
            in("rdx") 0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    if ret < 0 {
        Err(Error::Write(ret as i32))
    } else {
        Ok(Fd(ret as u32))
    }
}
