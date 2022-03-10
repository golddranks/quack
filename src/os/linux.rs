use core::arch::asm;
use crate::{os::Fd, Error};


#[repr(i32)]
enum Syscall {
    Read = 0x00000000,
    Write = 0x00000001,
    Open = 0x00000002,
    Close = 0x00000003,
    Mmap = 0x00000009,
    Exit = 0x00000060,
}

pub fn exit(ret: u8) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Exit as i64,
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
        Err(Error::Write(ret as i32))
    }
}

pub fn read(fd: Fd, buf: &mut [u8]) -> Result<usize, Error> {
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
        Err(Error::Write(ret as i32))
    } else {
        Ok(Fd(ret as u32))
    }
}