
use core::arch::asm;

use crate::os::{self, STDERR};
use crate::elf::parse::ProgHead;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
pub fn probe() {

    let x: u64;
    unsafe { asm!("lea {}, [rip]", out(reg) x); }
    writeln!(STDERR, "{:x?}", x);
    writeln!(STDERR, "{:x?}", probe as fn() as u64);
    writeln!(STDERR, "main: {:x?}", crate::main as fn() as u64);
}

#[cfg(all(target_os="linux", target_arch="x86_64"))]
pub fn load(phs: &[impl ProgHead], reader: &mut (impl Read + Seek)) {
    for ph in phs {
        let prog_count: u64;
        unsafe { asm!("lea {}, [rip]", out(reg) prog_count) };
        let range_to_be_loaded = ph.vaddr()..ph.vaddr()+ph.filesz();
        if range_to_be_loaded.contains(&(prog_count as usize)) {
            writeln!(STDERR, "what the hell {:?} {:?}", range_to_be_loaded, prog_count);
        } else {
            writeln!(STDERR, "hell yeah {:?} {:?}", range_to_be_loaded, prog_count);
        }
        unsafe {
            libc::mmap(ph.vaddr() as *mut c_void,
                ph.filesz(),
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
        }
    }
}