use core::{mem::size_of, slice::from_raw_parts_mut};
use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

mod enum_impls;
mod ffi_types;
#[cfg(test)]
mod test;

use crate::{e, Error};

use ffi_types::{
    Elf32Offs, Elf64Offs, ElfHead32, ElfHead64, ElfNonArchDep, ElfNonArchDep2, ProgHead32,
    ProgHead64, SectHead32, SectHead64, Sym32, Sym64,
};

use self::ffi_types::{EMachine, ShType};

trait ToKnown: TransmuteSafe {
    type Known;
    type Unknown;
    fn known(&self) -> Result<Self::Known, Self::Unknown>;
    fn unknown(&self) -> Self::Unknown;
}

pub unsafe trait TransmuteSafe: Default + Clone {
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        // This unsafe is sound because:
        // - Self is TransmuteSafe
        // - TransmuteSafe is an unsafe trait that guarantees that Self allows any byte pattern
        // - [u8] has alignment of 1, which is always less or equal than Self's alignment
        // - the size of [u8] is set to equal the size of Self in bytes
        // - The mutable access to the bytes of Self is constrained by the lifetime of &mut self
        // - Accepting &mut Self as an argument guarantees that its bytes are already initialized
        unsafe { from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    }
}

pub fn vec_as_bytes_mut<T: TransmuteSafe>(vec: &mut Vec<T>, n: usize) -> &mut [u8] {
    vec.clear();
    vec.resize(n, T::default());
    // This unsafe is sound because:
    // - vec is reserved to have a buffer that is large enough to fit the all Ts
    // - The buffer is filled with T::default() so all the bytes are initialized
    // - Self is TransmuteSafe
    // - TransmuteSafe is an unsafe trait that guarantees that Self allows any byte pattern
    // - [u8] has alignment of 1, which is always less or equal than Self's alignment
    // - The mutable access to the bytes of Self is constrained by the lifetime of &mut self
    unsafe { from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, n * size_of::<T>()) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Strings {
    buf: Vec<u8>,
}

trait ElfHead {
    type Offs: TransmuteSafe;
    type SectHead: TransmuteSafe;
    type ProgHead: TransmuteSafe;
    fn phoff(&self) -> u64;
    fn shoff(&self) -> u64;
    fn phnum(&self) -> usize;
    fn shnum(&self) -> usize;
    fn offs(reader: &mut impl Read) -> Result<Self::Offs, Error> {
        let mut offs = Self::Offs::default();
        reader.read_exact(offs.as_bytes_mut())?;
        Ok(offs)
    }

    fn tail(reader: &mut impl Read) -> Result<ElfNonArchDep2, Error>
    where
        Self: Sized,
    {
        let mut tail = ElfNonArchDep2::default();
        reader.read_exact(tail.as_bytes_mut())?;
        tail.check::<Self>()?;
        Ok(tail)
    }
    fn prog_headers(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<Self::ProgHead>, Error> {
        let mut v = Vec::new();
        reader.seek(SeekFrom::Start(self.phoff()))?;
        reader.read_exact(vec_as_bytes_mut(&mut v, self.phnum()))?;
        Ok(v)
    }

    fn sect_headers(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<Self::SectHead>, Error> {
        let mut v = Vec::new();
        reader.seek(SeekFrom::Start(self.shoff()))?;
        reader.read_exact(vec_as_bytes_mut(&mut v, self.shnum()))?;
        Ok(v)
    }
}

pub trait SectHead: Debug {
    type SymTab: TransmuteSafe;
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error>;
    fn sh_type(&self) -> Result<ShType, Error>;
    fn offset(&self) -> usize;
    fn size(&self) -> usize;
    fn entsize(&self) -> usize;
}

pub trait Sym: Debug {
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error>;
}

impl ElfHead for ElfHead32 {
    type Offs = Elf32Offs;
    type SectHead = SectHead32;
    type ProgHead = ProgHead32;
    fn phoff(&self) -> u64 {
        self.offs.e_phoff as u64
    }
    fn shoff(&self) -> u64 {
        self.offs.e_shoff as u64
    }
    fn phnum(&self) -> usize {
        self.tail.e_phnum as usize
    }
    fn shnum(&self) -> usize {
        self.tail.e_shnum as usize
    }
}

impl ElfHead for ElfHead64 {
    type Offs = Elf64Offs;
    type SectHead = SectHead64;
    type ProgHead = ProgHead64;
    fn phoff(&self) -> u64 {
        self.offs.e_phoff as u64
    }
    fn shoff(&self) -> u64 {
        self.offs.e_shoff as u64
    }
    fn phnum(&self) -> usize {
        self.tail.e_phnum as usize
    }
    fn shnum(&self) -> usize {
        self.tail.e_shnum as usize
    }
}

impl SectHead for SectHead32 {
    type SymTab = Sym32;
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error> {
        Ok(str.get_string(self.head.sh_name as usize)?)
    }
    fn sh_type(&self) -> Result<ShType, Error> {
        match self.head.sh_type.known() {
            Ok(o) => Ok(o),
            Err(_) => e("unknown sh_type"),
        }
    }
    fn offset(&self) -> usize {
        self.sh_offset as usize
    }
    fn size(&self) -> usize {
        self.sh_size as usize
    }
    fn entsize(&self) -> usize {
        self.sh_entsize as usize
    }
}

impl SectHead for SectHead64 {
    type SymTab = Sym64;
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error> {
        Ok(str.get_string(self.head.sh_name as usize)?)
    }
    fn sh_type(&self) -> Result<ShType, Error> {
        match self.head.sh_type.known() {
            Ok(o) => Ok(o),
            Err(_) => e("unknown sh_type"),
        }
    }
    fn offset(&self) -> usize {
        self.sh_offset as usize
    }
    fn size(&self) -> usize {
        self.sh_size as usize
    }
    fn entsize(&self) -> usize {
        self.sh_entsize as usize
    }
}

impl Sym for Sym32 {
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error> {
        str.get_string(self.st_name as usize)
    }
}

impl Sym for Sym64 {
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error> {
        str.get_string(self.st_name as usize)
    }
}

impl ElfNonArchDep {
    fn check(&self) -> Result<(), Error> {
        if self.e_ident.ei_mag != [0x7F, b'E', b'L', b'F'] {
            return e("invalid elf_header.e_ident.ei_mag");
        }
        match self.e_ident.ei_class {
            1 => return e("quack doesn't support 32-bit elfs"),
            2 => (),
            _ => return e("invalid elf_header.e_ident.ei_class"),
        }
        match self.e_ident.ei_data {
            1 => (),
            2 => return e("quack doesn't support big-endian elfs"),
            _ => return e("invalid elf_header.e_ident.ei_class"),
        }
        match self.e_ident.ei_version {
            1 => (),
            _ => return e("invalid elf_header.e_ident.ei_version"),
        }
        if self.e_ident.ei_pad != [0; 7] {
            return e("invalid elf.header.e_ident.ei_pad");
        }
        if self.e_machine.known() != Ok(EMachine::X86_64) {
            return e("quack doesn't support other archs than x86-64");
        }
        if self.e_version != 0x01 {
            return e("invalid elf_header.e_version");
        }
        Ok(())
    }
}

impl ElfNonArchDep2 {
    fn check<T: ElfHead>(&self) -> Result<(), Error> {
        if self.e_ehsize as usize != size_of::<T>() {
            return e("elf_header.e_ehsize invalid");
        }
        if self.e_phnum > 0 && self.e_phentsize as usize != size_of::<T::ProgHead>() {
            return e("elf_header.e_phentsize invalid");
        }
        if self.e_shnum > 0 && self.e_shentsize as usize != size_of::<T::SectHead>() {
            return e("elf_header.e_shentsize invalid");
        }
        Ok(())
    }
}

#[derive(Debug)]
enum ElfHeadType {
    EH32(ElfHead32),
    EH64(ElfHead64),
}

impl ElfHeadType {
    fn from(reader: &mut impl Read) -> Result<ElfHeadType, Error> {
        let mut head = ElfNonArchDep::default();
        reader.read_exact(head.as_bytes_mut())?;
        head.check()?;
        let eh = match head.e_ident.ei_class {
            1 => {
                let offs = ElfHead32::offs(reader)?;
                let tail = ElfHead32::tail(reader)?;
                ElfHeadType::EH32(ElfHead32 { head, offs, tail })
            }
            2 => {
                let offs = ElfHead64::offs(reader)?;
                let tail = ElfHead64::tail(reader)?;
                ElfHeadType::EH64(ElfHead64 { head, offs, tail })
            }
            _ => unreachable!(),
        };
        Ok(eh)
    }
}

impl Strings {
    fn from<T: SectHead>(reader: &mut (impl Read + Seek), str_head: &T) -> Result<Strings, Error> {
        if str_head.sh_type()? != ShType::Strtab {
            return e("invalid sh_type for a string section");
        }
        reader.seek(SeekFrom::Start(str_head.offset() as u64))?;
        let mut buf = Vec::new();
        buf.resize(str_head.size(), 0);
        reader.read_exact(&mut buf)?;

        if str_head.size() > 0 && (buf[0] != b'\0' || buf[buf.len() - 1] != b'\0') {
            return e("malformed string section");
        }

        Ok(Strings { buf })
    }

    pub fn get_string(&self, offset: usize) -> Result<&[u8], Error> {
        if offset == 0 {
            return Ok(&self.buf[0..1]);
        }
        if offset >= self.buf.len() {
            return e("invalid offset");
        }
        let end = offset
            + self.buf[offset..]
                .iter()
                .position(|&b| b == b'\0')
                .expect("null byte checked on init");
        Ok(&self.buf[offset..end])
    }
}

fn sh_names<T: SectHead>(
    reader: &mut (impl Read + Seek),
    eh_tail: &ElfNonArchDep2,
    shs: &[T],
) -> Result<Strings, Error> {
    let sh_strs = &shs[eh_tail.e_shstrndx as usize];
    Ok(Strings::from(reader, sh_strs)?)
}

fn find_sh_by<'a, T: SectHead>(
    shs: &'a [T],
    sh_names: &Strings,
    sh_type: ShType,
    name: &[u8],
) -> Result<Option<&'a T>, Error> {
    for sh in shs {
        if sh.sh_type()? == sh_type && sh.name(sh_names)? == name {
            return Ok(Some(sh));
        }
    }
    Ok(None)
}

fn symtab<T: SectHead>(
    reader: &mut (impl Read + Seek),
    shs: &[T],
    sh_names: &Strings,
) -> Result<Option<Vec<T::SymTab>>, Error> {
    if let Some(symtab) = find_sh_by(shs, sh_names, ShType::Symtab, b".symtab")? {
        if symtab.entsize() != size_of::<T::SymTab>() {
            return e("invalid symtab entity size");
        }
        reader.seek(SeekFrom::Start(symtab.offset() as u64))?;
        let mut syms: Vec<T::SymTab> = Vec::new();
        let n = symtab.size() / symtab.entsize();
        reader.read_exact(vec_as_bytes_mut(&mut syms, n))?;

        return Ok(Some(syms));
    }
    Ok(None)
}

fn sym_names<T: SectHead>(
    reader: &mut (impl Read + Seek),
    shs: &[T],
    sh_names: &Strings,
) -> Result<Option<Strings>, Error> {
    if let Some(strtab) = find_sh_by(shs, sh_names, ShType::Strtab, b".strtab")? {
        Ok(Some(Strings::from(reader, strtab)?))
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct ElfFile32 {
    pub eh: ElfHead32,
    pub phs: Vec<ProgHead32>,
    pub shs: Option<Vec<SectHead32>>,
    pub sh_names: Option<Strings>,
    pub symtab: Option<Vec<Sym32>>,
    pub sym_names: Option<Strings>,
}

#[derive(Debug, PartialEq)]
pub struct ElfFile64 {
    pub eh: ElfHead64,
    pub phs: Vec<ProgHead64>,
    pub shs: Option<Vec<SectHead64>>,
    pub sh_names: Option<Strings>,
    pub symtab: Option<Vec<Sym64>>,
    pub sym_names: Option<Strings>,
}

#[derive(Debug)]
pub enum ElfParse {
    Elf32(ElfFile32),
    Elf64(ElfFile64),
}

pub fn parse_elf(reader: &mut (impl Read + Seek)) -> Result<ElfParse, Error> {
    let elf_header = ElfHeadType::from(reader)?;
    match elf_header {
        ElfHeadType::EH32(eh) => {
            let phs = eh.prog_headers(reader)?;
            let shs = eh.sect_headers(reader)?;
            let sh_names = sh_names(reader, &eh.tail, &shs)?;
            let symtab = symtab(reader, &shs, &sh_names)?;
            let sym_names = sym_names(reader, &shs, &sh_names)?;
            Ok(ElfParse::Elf32(ElfFile32 {
                eh,
                phs,
                shs: Some(shs),
                sh_names: Some(sh_names),
                symtab,
                sym_names,
            }))
        }
        ElfHeadType::EH64(eh) => {
            let phs = eh.prog_headers(reader)?;
            let shs = eh.sect_headers(reader)?;
            let sh_names = sh_names(reader, &eh.tail, &shs)?;
            let symtab = symtab(reader, &shs, &sh_names)?;
            let sym_names = sym_names(reader, &shs, &sh_names)?;
            Ok(ElfParse::Elf64(ElfFile64 {
                eh,
                phs,
                shs: Some(shs),
                sh_names: Some(sh_names),
                symtab,
                sym_names,
            }))
        }
    }
}

pub fn parse_elf_from(path: &Path) -> Result<ElfParse, Error> {
    let f = File::open(path)?;
    Ok(parse_elf(&mut BufReader::new(f))?)
}
