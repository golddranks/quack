use core::{mem::size_of};
use std::{
    fmt::Debug,
    io::{Read, Seek, SeekFrom},
};

mod enum_impls;
mod ffi_types;
#[cfg(test)]
mod test;

use crate::{e, Error, utils::{ToKnown, TransmuteSafe, vec_as_bytes_mut}};

use ffi_types::{
    EIData, EIClass, PType, Elf32Offs, Elf64Offs, ElfHead32, ElfHead64, ElfNonArchDep, ElfNonArchDep2, ProgHead32,
    ProgHead64, SectHead32, SectHead64, Sym32, Sym64, EMachine, ShType,
};

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

pub trait ProgHead {
    fn p_type(&self) -> Result<PType, Error>;
    fn offset(&self) -> usize;
    fn vaddr(&self) -> usize;
    fn filesz(&self) -> usize;
    fn memsz(&self) -> usize;
    fn align(&self) -> usize;
}

impl ProgHead for ProgHead32 {
    fn p_type(&self) -> Result<PType, Error> {
        match self.p_type.known() {
            Ok(o) => Ok(o),
            Err(_) => e("invalid p_type value"),
        }
    }

    fn offset(&self) -> usize {
        self.p_offset as usize
    }

    fn vaddr(&self) -> usize {
        self.p_vaddr as usize
    }

    fn filesz(&self) -> usize {
        self.p_filesz as usize
    }

    fn memsz(&self) -> usize {
        self.p_memsz as usize
    }

    fn align(&self) -> usize {
        self.p_align as usize
    }
}

impl ProgHead for ProgHead64 {
    fn p_type(&self) -> Result<PType, Error> {
        match self.p_type.known() {
            Ok(o) => Ok(o),
            Err(_) => e("invalid p_type value"),
        }
    }

    fn offset(&self) -> usize {
        self.p_offset as usize
    }

    fn vaddr(&self) -> usize {
        self.p_vaddr as usize
    }

    fn filesz(&self) -> usize {
        self.p_filesz as usize
    }

    fn memsz(&self) -> usize {
        self.p_memsz as usize
    }

    fn align(&self) -> usize {
        self.p_align as usize
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StBind {
    Local = 0,
    Global = 1,
    Weak = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StType {
    NoType = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    Tls = 6,
}

pub trait Sym: Debug {
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error>;
    fn binding(&self) -> Result<StBind, Error>;
    fn st_type(&self) -> Result<StType, Error>;
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

fn st_type(st_info: u8) -> Result<StType, Error>  {
    Ok(match st_info & 0x0f {
        0 => StType::NoType,
        1 => StType::Object,
        2 => StType::Func,
        3 => StType::Section,
        4 => StType::File,
        5 => StType::Common,
        6 => StType::Tls,
        _ => return e("unknown symtab.st_type value"),
    })
}

fn st_bind(st_info: u8) -> Result<StBind, Error> {
    Ok(match st_info >> 4 {
        0 => StBind::Local,
        1 => StBind::Global,
        2 => StBind::Weak,
        _ => return e("unknown symtab.st_bind value"),
    })
}

impl Sym for Sym32 {
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error> {
        str.get_string(self.st_name as usize)
    }

    fn binding(&self) -> Result<StBind, Error> {
        st_bind(self.st_info)
    }

    fn st_type(&self) -> Result<StType, Error> {
        st_type(self.st_info)
    }
}

impl Sym for Sym64 {
    fn name<'a>(&self, str: &'a Strings) -> Result<&'a [u8], Error> {
        str.get_string(self.st_name as usize)
    }

    fn binding(&self) -> Result<StBind, Error> {
        st_bind(self.st_info)
    }

    fn st_type(&self) -> Result<StType, Error> {
        st_type(self.st_info)
    }
}

impl ElfNonArchDep {
    fn check(&self) -> Result<(), Error> {
        if self.e_ident.ei_mag != [0x7F, b'E', b'L', b'F'] {
            return e("invalid elf_header.e_ident.ei_mag");
        }
        match self.e_ident.ei_class.known() {
            Ok(EIClass::Elf32Bit) => return e("quack doesn't support 32-bit elfs"),
            Ok(EIClass::Elf64Bit) => (),
            Err(_) => return e("invalid elf_header.e_ident.ei_class"),
        }
        match self.e_ident.ei_data.known() {
            Ok(EIData::LittleEndian) => (),
            Ok(EIData::BigEndian) => return e("quack doesn't support big-endian elfs"),
            Err(_) => return e("invalid elf_header.e_ident.ei_class"),
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
        let eh = match head.e_ident.ei_class.known() {
            Ok(EIClass::Elf32Bit) => {
                let offs = ElfHead32::offs(reader)?;
                let tail = ElfHead32::tail(reader)?;
                ElfHeadType::EH32(ElfHead32 { head, offs, tail })
            }
            Ok(EIClass::Elf64Bit) => {
                let offs = ElfHead64::offs(reader)?;
                let tail = ElfHead64::tail(reader)?;
                ElfHeadType::EH64(ElfHead64 { head, offs, tail })
            }
            Err(_) => unreachable!(),
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
        if sh.sh_type() == Ok(sh_type) && sh.name(sh_names)? == name {
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

pub fn with(reader: &mut (impl Read + Seek)) -> Result<ElfParse, Error> {
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