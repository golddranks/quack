use core::{intrinsics::transmute, mem::size_of};
use std::{path::Path, fs::File, io::{Read, BufReader, Seek, SeekFrom}, ops::Range};

use crate::{Error, e};

#[repr(C)]
#[derive(Default, Debug)]
struct EIdent {
    ei_mag: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 7],
}

#[repr(C)]
#[derive(Default, Debug)]
struct ElfNonArchDep {
    e_ident: EIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
}

impl ElfNonArchDep {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
struct ElfNonArchDep2 {
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl ElfNonArchDep2 {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
struct Elf32Offs {
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
}

impl Elf32Offs {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
struct Elf64Offs {
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
}

impl Elf64Offs {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

#[derive(Debug)]
pub struct ElfHead32 {
    head: ElfNonArchDep,
    offs: Elf32Offs,
    tail: ElfNonArchDep2,
}

#[derive(Debug)]
pub struct ElfHead64 {
    head: ElfNonArchDep,
    offs: Elf64Offs,
    tail: ElfNonArchDep2,
}

trait ElfHead {
    type SectHead;
    type ProgHead;
}

impl ElfHead for ElfHead32 {
    type SectHead = SectHead32;
    type ProgHead = ProgHead32;
}

impl ElfHead for ElfHead64 {
    type SectHead = SectHead64;
    type ProgHead = ProgHead64;
}

#[derive(Debug)]
enum ElfArchDep {
    EH32(ElfHead32),
    EH64(ElfHead64),
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
        if self.e_machine != 0x3e {
            return e("quack doesn't support other archs than x86-64");
        }
        if self.e_version != 0x01 {
            return e("invalid elf_header.e_version");
        }
        Ok(())
    }
}

impl ElfArchDep {
    fn tail(reader: &mut impl Read) -> Result<ElfNonArchDep2, Error> {
        let mut tail = ElfNonArchDep2::default();
        reader.read_exact(tail.as_slice_mut())?;
        Ok(tail)
    }

    fn from(reader: &mut impl Read) -> Result<ElfArchDep, Error> {
        let mut head = ElfNonArchDep::default();
        reader.read_exact(head.as_slice_mut())?;
        head.check()?;  
        let eh = match head.e_ident.ei_class {
            1 => {
                let mut offs = Elf32Offs::default();
                reader.read_exact(offs.as_slice_mut())?;
                let tail = Self::tail(reader)?;
                tail.check::<ElfHead32>()?;
                ElfArchDep::EH32(ElfHead32 {
                    head, offs, tail,
                })
            },
            2 => {
                let mut offs = Elf64Offs::default();
                reader.read_exact(offs.as_slice_mut())?;
                let tail = Self::tail(reader)?;
                tail.check::<ElfHead64>()?;
                ElfArchDep::EH64(ElfHead64 {
                    head, offs, tail,
                })
            },
            _ => unreachable!(),
        };
        Ok(eh)
    }
}

impl ElfHead32 {
    fn prog_headers(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<ProgHead32>, Error> {
        let mut v = Vec::with_capacity(self.tail.e_phnum as usize);
        reader.seek(SeekFrom::Start(self.offs.e_phoff as u64))?;
        for _ in 0..self.tail.e_phnum {
            let mut ph = ProgHead32::default();
            reader.read_exact(ph.as_slice_mut())?;
            v.push(ph);
        }
        Ok(v)
    }

    fn sect_headers(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<SectHead32>, Error> {
        let mut v = Vec::with_capacity(self.tail.e_shnum as usize);
        reader.seek(SeekFrom::Start(self.offs.e_shoff as u64))?;
        for _ in 0..self.tail.e_shnum {
            let mut sh = SectHead32::default();
            reader.read_exact(sh.as_slice_mut())?;
            v.push(sh);
        }
        let shstrs = &v[self.tail.e_shstrndx as usize];
        println!("SectHead strings: {:?}", shstrs);
        Ok(v)
    }
}
impl ElfHead64 {
    fn prog_headers(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<ProgHead64>, Error> {
        let mut v = Vec::with_capacity(self.tail.e_phnum as usize);
        reader.seek(SeekFrom::Start(self.offs.e_phoff as u64))?;
        for _ in 0..self.tail.e_phnum {
            let mut ph = ProgHead64::default();
            reader.read_exact(ph.as_slice_mut())?;
            v.push(ph);
        }
        Ok(v)
    }

    fn sect_headers(&self, reader: &mut (impl Read + Seek)) -> Result<Vec<SectHead64>, Error> {
        let mut v = Vec::with_capacity(self.tail.e_shnum as usize);
        reader.seek(SeekFrom::Start(self.offs.e_shoff as u64))?;
        for _ in 0..self.tail.e_shnum {
            let mut sh = SectHead64::default();
            reader.read_exact(sh.as_slice_mut())?;
            v.push(sh);
        }
        Ok(v)
    }
}

fn sh_names<T: SectHead>(eh_tail: &ElfNonArchDep2, sh: &[T], reader: &mut (impl Read + Seek)) -> Result<(Vec<Range<usize>>, Vec<u8>), Error> {
    let shstrs = &sh[eh_tail.e_shstrndx as usize];
    if shstrs.sh_type() != 3 {
        return e("e_shstrndx and shstr section sh_type don't match");
    }
    reader.seek(SeekFrom::Start(shstrs.offset() as u64))?;
    let mut strs = Vec::new();
    strs.resize(shstrs.size() as usize, 0);
    reader.read_exact(&mut strs)?;
    let mut ranges = Vec::new();
    let mut l = 0;
    for str in strs.split_inclusive(|&b| b == b'\0') {
        let r = l + str.len();
        ranges.push(l..r);
        l = r;
    }
    Ok((ranges, strs))
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct ProgHead64 {
    p_type: u32,
    p_flag: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct ProgHead32 {
    p_type: u32,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flag: u32,
    p_align: u32,
}

impl ProgHead32 {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

impl ProgHead64 {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct SectNonArchDep {
    sh_name: u32,
    sh_type: u32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct SectHead32 {
    head: SectNonArchDep,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct SectHead64 {
    head: SectNonArchDep,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

trait SectHead {
    fn name(&self) -> usize;
    fn sh_type(&self) -> usize;
    fn offset(&self) -> usize;
    fn size(&self) -> usize;
}

impl SectHead for SectHead32 {
    fn name(&self) -> usize {
        self.head.sh_name as usize
    }
    fn sh_type(&self) -> usize{
        self.head.sh_type as usize
    }
    fn offset(&self) -> usize{
        self.sh_offset as usize
    }
    fn size(&self) -> usize{
        self.sh_size as usize
    }
}

impl SectHead for SectHead64 {
    fn name(&self) -> usize {
        self.head.sh_name as usize
    }
    fn sh_type(&self) -> usize{
        self.head.sh_type as usize
    }
    fn offset(&self) -> usize{
        self.sh_offset as usize
    }
    fn size(&self) -> usize{
        self.sh_size as usize
    }
}

impl SectHead32 {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}

impl SectHead64 {
    fn as_slice_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
        unsafe { transmute(self) }
    }
}


pub enum ElfParse {
    Elf32(ElfHead32, Vec<ProgHead32>, Vec<SectHead32>, Vec<Range<usize>>, Vec<u8>),
    Elf64(ElfHead64, Vec<ProgHead64>, Vec<SectHead64>, Vec<Range<usize>>, Vec<u8>),
}


pub fn parse_elf(reader: &mut (impl Read + Seek)) -> Result<ElfParse, Error> {
    let elf_header = ElfArchDep::from(reader)?;
    match elf_header {
        ElfArchDep::EH32(eh) => {
            let phs = eh.prog_headers(reader)?;
            let shs = eh.sect_headers(reader)?;
            let (sh_name_ranges, sh_name_str) = sh_names(&eh.tail, &shs, reader)?;
            Ok(ElfParse::Elf32(eh, phs, shs, sh_name_ranges, sh_name_str))
        },
        ElfArchDep::EH64(eh) => {
            let phs = eh.prog_headers(reader)?;
            let shs = eh.sect_headers(reader)?;
            let (sh_name_ranges, sh_name_str) = sh_names(&eh.tail, &shs, reader)?;
            Ok(ElfParse::Elf64(eh, phs, shs, sh_name_ranges, sh_name_str))
        },
    }
}

pub fn parse_elf_from(path: &Path) -> Result<ElfParse, Error> {
    let f = File::open(path)?;
    Ok(parse_elf(&mut BufReader::new(f))?)
}