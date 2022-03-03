use core::{ptr::null_mut, mem, slice::from_raw_parts_mut};
use std::{io::{self, Read}, path::Path, env::args_os, fs::File};

use crate::elf::{parse_elf_from, ElfParse};

mod elf;

const RET: i32 = 0xc3;

struct RunMem {
    ptr: *mut u8,
    size: usize,
}

impl RunMem {
    fn new(n_pages: usize) -> RunMem {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        let mut page : *mut libc::c_void = null_mut();
        let size = page_size * n_pages;
        unsafe {
            libc::posix_memalign(&mut page, page_size, size);
            libc::mprotect(page, size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
            libc::memset(page, RET, size);
        }
    
        RunMem {
            ptr: page as *mut u8,
            size,
        }
    }

    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.ptr, self.size) }
    }

    fn to_fn_ptr(&self) -> unsafe fn() -> i64 { // TODO: return type?!
        unsafe { mem::transmute(self.ptr) }
    }
}

fn maps() -> Result<(), Error> {
    let mut maps = File::open("/proc/self/maps")?;
    let mut buf = Vec::new();
    maps.read_to_end(&mut buf)?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
} 

fn main() -> Result<(), Error> {

    #[cfg(target_os = "linux")]
    maps()?;

    let mut mem = RunMem::new(1);
    let slice = mem.as_slice_mut();
    slice[0] = 0x48;
    slice[1] = 0xC7;
    slice[2] = 0xC0;
    slice[3] = 0x03;
    slice[4] = 0x00;
    slice[5] = 0x00;
    slice[6] = 0x00;
    slice[7] = 0xC3;

    let func = mem.to_fn_ptr();

    println!("Called: {}", unsafe { func() });

    let path = args_os().nth(1).ok_or(Error::new("provide a binary file"))?;

    let elf = parse_elf_from(Path::new(&path))?;
    match elf {
        ElfParse::Elf32(_, _, _, _, _) => unimplemented!(),
        ElfParse::Elf64(eh, phs, shs, sh_name_ranges, sh_name_str) => {
            println!("{:?}", eh);
            for ph in phs {
                println!("{:?}", ph);
            }
            for (sh, range) in shs.iter().zip(sh_name_ranges) {
                println!("{} {:?}", String::from_utf8_lossy(&sh_name_str[range]), sh);
            }
        },
    }
    Ok(())
}

#[derive(Debug)]
pub struct Error {
    s: &'static str,
}

impl Error {
    fn new(s: &'static str) -> Error {
        Error {
            s,
        }
    }
}

fn e<T>(s: &'static str) -> Result<T, Error> {
    Err(Error::new(s))
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error {
            s: "io::Error",
        }
    }
}
