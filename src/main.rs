#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::{fmt::Write};

mod error;
mod os;

use elf::e;
use error::Error;

/*

pub fn main(args: os::Args) -> Result<(), Error> {
    os::write(os::STDERR, "main start\n")?;

    for i in 0..args.len() {
        os::write(os::STDERR, args.nth(i))?;
        os::write(os::STDERR, "\n")?;
    }

    let log = os::open_for_log("test/trace.log\0")?;
    writeln!(os::STDERR, "{:?}", log)?;
    os::write(log, "test")?;

    let elf = os::open_for_read("test/hello.txt\0")?;
    writeln!(os::STDERR, "{:?}", elf)?;
    let slice = os::map_file(elf)?;
    os::write(os::STDERR, slice.as_slice())?;

    os::write(os::STDERR, "main end\n")?;
    Ok(())
}

*/

mod elf;
mod utils;

use crate::elf::parse::{ElfFile64, ElfParse, Sym, StType};

// TODO:
// Load and run ELF
// Patch symbols
// Rust demangling
// Mach-O support

fn main(args: os::Args) -> Result<(), Error> {
    if args.len() < 2 {
        writeln!(os::STDERR, "Provide a path to binary file as the first argument!")?;
        return Err(Error::Cli)
    }
    let path = args.nth(1);

    //#[cfg(all(target_os="linux", target_arch="x86_64"))]
    //runmem::maps();

    let elf_fd = os::open_for_read(path)?;
    let elf_file = os::map_file(elf_fd)?;
    writeln!(os::STDERR, "elf_file: {:?} size: {}", elf_file.as_slice().as_ptr(), elf_file.as_slice().len())?;
    let elf = elf::parse::with(elf_file.as_slice())?;
    match elf {
        ElfParse::Elf32(_) => unimplemented!(),
        ElfParse::Elf64(ElfFile64 {
            eh,
            phs,
            shs: Some(_shs),
            sh_names: Some(_sh_names),
            symtab: Some(symtab),
            sym_names: Some(sym_names),
        }) => {
            writeln!(os::STDERR, "eh: {:?}", eh)?;
            for ph in phs {
                writeln!(os::STDERR, "ph: {:?}", ph)?;
            }
            #[cfg(all(target_os="linux", target_arch="x86_64"))]
            elf::load::probe();
            #[cfg(all(target_os="linux", target_arch="x86_64"))]
            elf::load::load(&phs, &mut reader);
            for sym in symtab {
                if let Ok(StType::Func) = sym.st_type() {
                    writeln!(os::STDERR,
                        "{} {:?}",
                        core::str::from_utf8(sym.name(&sym_names)?)?,
                        sym.binding()?
                    )?;
                }
            }
        }
        _ => return e("no elf with symbol table etc."),
    }
    Ok(())
}