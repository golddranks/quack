use core::{
    arch::{asm, global_asm}, ffi::c_void, slice
};

use crate::{os::{Fd, MappedFile, STDERR}, Error};

global_asm!("
.globl start
start:      # entry point of the binary, called by the loader
    pop     rdi  # stack points to argc; pop that & pass it to start2 as the 1st arg (rdi)
    mov     rsi, rsp  # stack (rsi) points to argv; pass it to start2 as the 2nd arg (rsi)
    and     rsp, 0xfffffffffffffff0 # align stack to 16 bytes; expected by the ABI
    call    _start2"
);

#[repr(u32)]
enum Syscall {
    Exit = 0x02000001,
    Read = 0x02000003,
    Write = 0x02000004,
    Open = 0x02000005,
    Close = 0x02000006,
    Mmap = 0x020000C5,
    Fstat64 = 0x02000153,
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
    pub const RD_ONLY: i32 = 0x0000;
    pub const WR_ONLY: i32 = 0x0001;
    pub const RD_WR: i32 = 0x0002;
    pub const CREAT: i32 = 0x0200;
    pub const APPEND: i32 = 0x0008;
} 

const AX_CARRY_BIT: u16 = 0x0100;

pub fn open(path: &[u8], mode: i32, file_perms: i32) -> Result<Fd, Error> {
    if let Some(b'\0') = path.last() {
    } else {
        panic!("path must be null-terminated");
    };
    let ret: i64;
    let flags: u16;
    unsafe {
        asm!(
            "syscall",
            "mov rcx, rax", // move the return value away from rax
            "lahf", // check the carry flag, which MacOS uses to report error status
            in("rax") Syscall::Open as u32,
            in("rdi") path.as_ptr(),
            in("rsi") mode,
            in("rdx") file_perms,
            out("rcx") ret,
            out("r11") _,
            lateout("ax") flags,
        );
    }
    if flags & AX_CARRY_BIT == 0 {
        Ok(Fd(ret as u32))
    } else {
        Err(Error::Open(ret as i32))
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
struct TimeSpec {
    sec: u64,
    nsec: u64,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stat64 {
    dev: i32,               /* ID of device containing file */
    mode: u16,              /* Mode of file (see below) */
    nlink: u16,             /* Number of hard links */
    ino: u64,               /* File serial number */
    uid: u32,               /* User ID of the file */
    gid: u32,               /* Group ID of the file */
    rdev: i32,              /* Device ID */
    padding: i32,
    atimespec: TimeSpec,     /* time of last access */
    mtimespec: TimeSpec,     /* time of last data modification */
    ctimespec: TimeSpec,     /* time of last status change */
    birthtimespec: TimeSpec, /* time of file creation(birth) */
    pub (crate) size: i64,   /* file size, in bytes */
    blocks: i64,             /* blocks allocated for file */
    blksize: i32,            /* optimal blocksize for I/O */
    flags: u32,              /* user defined flags for file */
    gen: u32,                /* file generation number */
    lspare: i32,            /* RESERVED: DO NOT USE! */
    qspare: [u64; 2],       /* RESERVED: DO NOT USE! */
}

#[test]
fn test_stat_layout() {
    use core::mem::{size_of, align_of};
    assert_eq!(size_of::<Stat64>(), 144);
    assert_eq!(align_of::<Stat64>(), 8);
}

pub fn fstat(fd: Fd) -> Result<Stat64, Error> {
    let mut stat = Stat64::default();
    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Fstat64 as u32,
            in("rdi") fd.0,
            in("rsi") &mut stat,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        )
    }
    if ret < 0 {
        Err(Error::Fstat(ret as i32))
    } else {
        Ok(stat)
    }
}

pub mod MmapProt {
    pub const PROT_NONE: u32 =  0x00;    /* [MC2] no permissions */
    pub const PROT_READ: u32 =  0x01;    /* [MC2] pages can be read */
    pub const PROT_WRITE: u32 = 0x02;    /* [MC2] pages can be written */
    pub const PROT_EXEC: u32 =  0x04;    /* [MC2] pages can be executed */
}

pub mod MmapFlags {
    pub const MAP_SHARED: u32 =  0x0001;    /* [MF|SHM] share changes */
    pub const MAP_PRIVATE: u32 =  0x0002;   /* [MF|SHM] changes are private */
    pub const MAP_FIXED: u32 = 0x0010;      /* [MF|SHM] interpret addr exactly */
    pub const MAP_ANON: u32 =  0x1000;      /* allocated from memory, swap space */
}

pub fn mmap(addr: *const c_void, len: i64, prot: u32, flags: u32, fd: Fd, offset: u64) -> Result<MappedFile, Error> {
    assert!(prot & MmapProt::PROT_READ != 0);
    let ret: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") Syscall::Mmap as u32,
            in("rdi") addr,
            in("rsi") len,
            in("rdx") prot,
            inout("rcx") flags => _,
            in("r8") fd.0,
            in("r9") offset,
            out("r11") _,
            lateout("rax") ret,
        )
    }
    if ret < 0 {
        Err(Error::Open(ret as i32))
    } else {
        if prot & MmapProt::PROT_WRITE != 0 {
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