use std::{mem, fs::File, ptr::null_mut, slice::from_raw_parts_mut, io::Read};

use crate::Error;

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

pub fn maps() -> Result<(), Error> {
    let mut maps = File::open("/proc/self/maps")?;
    let mut buf = Vec::new();
    maps.read_to_end(&mut buf)?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
} 

fn run() {

/*
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
*/
}