use core::{
    arch::{asm, global_asm},
};

use crate::{os::Fd, Error};

global_asm!("
.globl _start
_start:     # entry point of the binary, called by the loader
    pop     rdi  # stack points to argc; pop that & pass it to start2 as the 1st arg (rdi)
    mov     rsi, rsp  # stack (rsi) points to argv; pass it to start2 as the 2nd arg (rsi)
    and     rsp, 0xfffffffffffffff0 # align stack to 16 bytes; expected by the ABI
    call    start2"
);

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

pub mod OpenMode {
    pub const RD_ONLY: i32 = 0x000;
    pub const WR_ONLY: i32 = 0x001;
    pub const RD_WR: i32 = 0x002;
    pub const CREAT: i32 = 0x100;
    pub const APPEND: i32 = 0x2000;
} 


pub fn open(path: &[u8], mode: i32, file_perms: i32) -> Result<Fd, Error> {
    if let Some(b'\0') = path.last() {
    } else {
        panic!("path must be null-terminated");
    };
    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Open as u32,
            in("rdi") path.as_ptr(),
            in("rsi") mode,
            in("rdx") file_perms,
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
