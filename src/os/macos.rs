use core::arch::asm;
use crate::{os::Fd, Error};


#[repr(u32)]
enum Syscall {
    Exit = 0x02000001,
    Read = 0x02000003,
    Write = 0x02000004,
    Open = 0x02000005,
    Close = 0x02000006,
    Mmap = 0x020000C5,
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
    if let Some(b'\0') = path.last() {} else {
        panic!("path must be null-terminated");
    };
    let ret: i32;
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
        Err(Error::Open(ret))
    } else {
        Ok(Fd(ret as u32))
    }
}