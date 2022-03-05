use std::{env::args_os, io, path::Path};

use crate::elf::{parse_elf_from, ElfFile64, ElfParse, SectHead, Sym, StType};

mod elf;

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

    let elf = parse_elf_from(Path::new(&path))?;
    match elf {
        ElfParse::Elf32(_) => unimplemented!(),
        ElfParse::Elf64(ElfFile64 {
            eh,
            phs,
            shs: Some(shs),
            sh_names: Some(sh_names),
            symtab: Some(symtab),
            sym_names: Some(sym_names),
        }) => {
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

#[derive(Debug)]
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
