#![no_std]
#![no_main]

#[cfg(all(target_os="linux", target_arch="x86_64"))]
mod linux;

pub struct Error;

pub fn main() -> Result<(), Error> {
    let file = linux::open("/proc/self/maps\0", linux::OpenMode::RdOnly)?;
    let mut buf = [0; 2000];
    linux::read(file, &mut buf[..])?;
    linux::write(linux::STDOUT, buf)?;

    linux::write(linux::STDOUT, "moikkaaaaa\n")?;
    linux::write(linux::STDOUT, "moikkaaaaa\n")?;
    Ok(())
}

/*
use std::{env::args_os, io, fs::File};

mod mmap;
mod elf;
mod utils;
mod runmem;

use crate::elf::parse::{ElfFile64, ElfParse, Sym, StType};

// TODO:
// Load and run ELF
// Patch symbols
// Rust demangling
// Mach-O support

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e.s)
        }
    }
}

fn run() -> Result<(), Error> {
    let path = args_os()
        .nth(1)
        .ok_or(Error::new("provide a binary file"))?;
    
    #[cfg(all(target_os="linux", target_arch="x86_64"))]
    runmem::maps();

    let mut reader = File::open(path)?;
    let elf = elf::parse::with(&mut reader)?;
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
            println!("eh: {:?}", eh);
            for ph in &phs {
                println!("ph: {:?}", ph);
            }
            #[cfg(target_arch="x86_64")]
            elf::load::probe();
            #[cfg(all(target_os="linux", target_arch="x86_64"))]
            elf::load::load(&phs, &mut reader);
            for sym in symtab {
                if let Ok(StType::Func) = sym.st_type() {
                    println!(
                        "{} {:?}",
                        String::from_utf8_lossy(sym.name(&sym_names)?),
                        sym.binding()?
                    );
                }
            }
        }
        _ => return e("no elf with symbol table etc."),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    s: &'static str,
}

impl Error {
    fn new(s: &'static str) -> Error {
        Error { s }
    }
}

fn e<T>(s: &'static str) -> Result<T, Error> {
    Err(Error::new(s))
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error { s: "io::Error" }
    }
}
*/