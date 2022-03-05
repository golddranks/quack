use std::io::Cursor;

use crate::elf::{self, parse::ElfParse};

#[test]
fn elf_loading() {
    let elf_file = &mut Cursor::new(include_bytes!("../../../test.musl.elf"));
    let parsed_elf = if let ElfParse::Elf64(elf64) = elf::parse::with(elf_file).unwrap() {
        elf64
    } else {
        unreachable!();
    };
    assert_eq!(parsed_elf.symtab.unwrap().len(), 147);
}
