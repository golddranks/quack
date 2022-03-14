use core::{
    arch::{asm, global_asm}, ffi::c_void, slice,
};

use crate::{os::{Fd, MappedFile}, Error};

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
    Fstat = 5,
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
        Err(Error::Open(ret as i32))
    } else {
        Ok(Fd(ret as u32))
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stat {
    dev: u64,                   /* ID of device containing file */
    ino: u64,                   /* inode number */
    mode: u32,                  /* protection */
    nlink: u64,                 /* number of hard links */
    uid: u32,                   /* user ID of owner */
    gid: u32,                   /* group ID of owner */
    rdev: u64,                  /* device ID (if special file) */
    pub (crate) size: i64,      /* total size, in bytes */
    blksize: i64,               /* blocksize for file system I/O */
    blocks: i64,                /* number of 512B blocks allocated */
    atime: u64,                 /* time of last access */
    mtime: u64,                 /* time of last modification */
    ctime: u64,                 /* time of last status change */
}

pub fn fstat(fd: Fd) -> Result<Stat, Error> {
    let mut stat = Stat::default();
    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Fstat as u32,
            in("rdi") fd.0,
            in("rsi") &mut stat,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    if ret < 0 {
        Err(Error::Fstat(ret as i32))
    } else {
        Ok(stat)
    }
}

pub mod mmap_prot {
    pub const PROT_NONE: u32 =  0x00;    /* Page can not be accessed.  */
    pub const PROT_READ: u32 =  0x01;    /* Page can be read.  */
    pub const PROT_WRITE: u32 = 0x02;    /* Page can be written.  */
    pub const PROT_EXEC: u32 =  0x04;    /* Page can be executed.  */
}

pub mod mmap_flags {
    pub const MAP_SHARED: u32 =  0x0001;    /* Share changes.  */
    pub const MAP_PRIVATE: u32 =  0x0002;   /* Changes are private.  */
    pub const MAP_FIXED: u32 = 0x0010;      /* Interpret addr exactly.  */
    pub const MAP_ANON: u32 =  0x0020;      /* Don't use a file.  */
}

pub fn mmap(addr: *const c_void, len: i64, prot: u32, flags: u32, fd: Fd, offset: u64) -> Result<MappedFile, Error> {
    assert!(prot & mmap_prot::PROT_READ != 0);
    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Mmap as u32,
            in("rdi") addr,
            in("rsi") len,
            in("rdx") prot,
            in("r10") flags,
            in("r8") fd.0,
            in("r9") 0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    if ret < 0 {
        Err(Error::Mmap(ret as i32))
    } else {
        if prot & mmap_prot::PROT_WRITE != 0 {
            Ok(MappedFile::ReadWrite(
                unsafe { slice::from_raw_parts_mut(ret as *mut u8, len as usize) }
            ))
        } else {
            Ok(MappedFile::ReadOnly(
                unsafe { slice::from_raw_parts(ret as *const u8, len as usize) }
            ))
        }
    }
}