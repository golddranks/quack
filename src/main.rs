use std::{io, path::Path, env::args_os};

use crate::elf::{parse_elf_from, ElfParse, ElfFile64, SectHead, Sym};

mod elf;

fn main() -> Result<(), Error> {
    let path = args_os().nth(1).ok_or(Error::new("provide a binary file"))?;

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
            println!("{:?}", eh);
            for ph in phs {
                println!("{:?}", ph);
            }
            for sh in shs {
                println!("{} {:?}", String::from_utf8_lossy(sh.name(&sh_names)?), sh);
            }
            for sym in symtab {
                println!("{} {:?}", String::from_utf8_lossy(sym.name(&sym_names)?), sym);
            }
        },
        e => eprintln!("No elf with symbol table etc: {:?}", e),
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
