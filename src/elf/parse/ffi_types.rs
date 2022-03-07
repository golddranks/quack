use std::fmt::Debug;

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EIdent {
    pub(super) ei_mag: [u8; 4],
    pub(super) ei_class: EIClassUnchecked,
    pub(super) ei_data: EIDataUnchecked,
    pub(super) ei_version: u8,
    pub(super) ei_osabi: EIOsAbiUnchecked,
    pub(super) ei_abiversion: u8,
    pub(super) ei_pad: [u8; 7],
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EIClass {
    Elf32Bit = 0x01,
    Elf64Bit = 0x02,
}

#[derive(Copy, Clone)]
pub union EIClassUnchecked {
    pub(super) unknown: u8,
    pub(super) known: EIClass,
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EIData {
    LittleEndian = 0x01,
    BigEndian = 0x02,
}

#[derive(Copy, Clone)]
pub union EIDataUnchecked {
    pub(super) unknown: u8,
    pub(super) known: EIData,
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EIOsAbi {
    SystemV = 0x00,
    Linux = 0x03,
}

#[derive(Copy, Clone)]
pub union EIOsAbiUnchecked {
    pub(super) unknown: u8,
    pub(super) known: EIOsAbi,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElfNonArchDep {
    pub(super) e_ident: EIdent,
    pub(super) e_type: ETypeUnchecked,
    pub(super) e_machine: EMachineUnchecked,
    pub(super) e_version: u32,
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EType {
    None = 0x00,
    Rel = 0x01,
    Exec = 0x02,
    Dyn = 0x03,
    Core = 0x04,
}

#[derive(Copy, Clone)]
pub union ETypeUnchecked {
    pub(super) unknown: u16,
    pub(super) known: EType,
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EMachine {
    X86 = 0x03,
    X86_64 = 0x3E,
    Aarch64 = 0xB7,
}

#[derive(Copy, Clone)]
pub union EMachineUnchecked {
    pub(super) unknown: u16,
    pub(super) known: EMachine,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Elf32Offs {
    pub(super) e_entry: u32,
    pub(super) e_phoff: u32,
    pub(super) e_shoff: u32,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Elf64Offs {
    pub(super) e_entry: u64,
    pub(super) e_phoff: u64,
    pub(super) e_shoff: u64,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElfNonArchDep2 {
    pub(super) e_flags: u32,
    pub(super) e_ehsize: u16,
    pub(super) e_phentsize: u16,
    pub(super) e_phnum: u16,
    pub(super) e_shentsize: u16,
    pub(super) e_shnum: u16,
    pub(super) e_shstrndx: u16,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElfHead32 {
    pub(super) head: ElfNonArchDep,
    pub(super) offs: Elf32Offs,
    pub(super) tail: ElfNonArchDep2,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElfHead64 {
    pub(super) head: ElfNonArchDep,
    pub(super) offs: Elf64Offs,
    pub(super) tail: ElfNonArchDep2,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProgHead64 {
    pub(super) p_type: PTypeUnchecked,
    pub(super) p_flag: u32,
    pub(super) p_offset: u64,
    pub(super) p_vaddr: u64,
    pub(super) p_paddr: u64,
    pub(super) p_filesz: u64,
    pub(super) p_memsz: u64,
    pub(super) p_align: u64,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProgHead32 {
    pub(super) p_type: PTypeUnchecked,
    pub(super) p_offset: u32,
    pub(super) p_vaddr: u32,
    pub(super) p_paddr: u32,
    pub(super) p_filesz: u32,
    pub(super) p_memsz: u32,
    pub(super) p_flag: u32,
    pub(super) p_align: u32,
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PType {
    Null = 0x00000000,
    Load = 0x00000001,
    Dynamic = 0x00000002,
    Interp = 0x00000003,
    Note = 0x00000004,
    Shlib = 0x00000005,
    Phdr = 0x00000006,
    Tls = 0x00000007,
}

#[derive(Copy, Clone)]
pub union PTypeUnchecked {
    pub(super) unknown: u32,
    pub(super) known: PType,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SectNonArchDep {
    pub(super) sh_name: u32,
    pub(super) sh_type: ShTypeUnchecked,
}

#[allow(dead_code)] // These are actually constructed via type re-interpretation
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShType {
    Null = 0x0,
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0x0A,
    Dynsym = 0x0B,
    InitArray = 0x0E,
    FiniArray = 0x0F,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    Num = 0x13,
}

#[derive(Copy, Clone)]
pub union ShTypeUnchecked {
    pub(super) unknown: u32,
    pub(super) known: ShType,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SectHead32 {
    pub(super) head: SectNonArchDep,
    pub(super) sh_flags: u32,
    pub(super) sh_addr: u32,
    pub(super) sh_offset: u32,
    pub(super) sh_size: u32,
    pub(super) sh_link: u32,
    pub(super) sh_info: u32,
    pub(super) sh_addralign: u32,
    pub(super) sh_entsize: u32,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SectHead64 {
    pub(super) head: SectNonArchDep,
    pub(super) sh_flags: u64,
    pub(super) sh_addr: u64,
    pub(super) sh_offset: u64,
    pub(super) sh_size: u64,
    pub(super) sh_link: u32,
    pub(super) sh_info: u32,
    pub(super) sh_addralign: u64,
    pub(super) sh_entsize: u64,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sym32 {
    pub(super) st_name: u32,
    pub(super) st_value: u32,
    pub(super) st_size: u32,
    pub(super) st_info: u8,
    pub(super) st_other: u8,
    pub(super) st_shndx: u16,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Sym64 {
    pub(super) st_name: u32,
    pub(super) st_info: u8,
    pub(super) st_other: u8,
    pub(super) st_shndx: u16,
    pub(super) st_value: u64,
    pub(super) st_size: u64,
}

// These unsafe implementations are sound, because each of the implemeting types
// - are repr(C)
// - don't contain any gaps in theyr memory layout
// - consists only of types with no
use crate::utils::TransmuteSafe;


unsafe impl TransmuteSafe for ElfNonArchDep {}
unsafe impl TransmuteSafe for Elf32Offs {}
unsafe impl TransmuteSafe for Elf64Offs {}
unsafe impl TransmuteSafe for ElfNonArchDep2 {}
unsafe impl TransmuteSafe for ProgHead32 {}
unsafe impl TransmuteSafe for ProgHead64 {}
unsafe impl TransmuteSafe for SectHead32 {}
unsafe impl TransmuteSafe for SectHead64 {}
unsafe impl TransmuteSafe for Sym32 {}
unsafe impl TransmuteSafe for Sym64 {}

unsafe impl TransmuteSafe for EIClassUnchecked {}
unsafe impl TransmuteSafe for EIDataUnchecked {}
unsafe impl TransmuteSafe for EIOsAbiUnchecked {}
unsafe impl TransmuteSafe for ETypeUnchecked {}
unsafe impl TransmuteSafe for EMachineUnchecked {}
unsafe impl TransmuteSafe for PTypeUnchecked {}
unsafe impl TransmuteSafe for ShTypeUnchecked {}

// To ensure that there isn't any accidental padding etc.
#[test]
fn sizes_and_alignments() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<EIdent>(), 1);
    assert_eq!(align_of::<ElfNonArchDep>(), 4);
    assert_eq!(align_of::<Elf32Offs>(), 4);
    assert_eq!(align_of::<Elf64Offs>(), 8);
    assert_eq!(align_of::<ElfNonArchDep2>(), 4);
    assert_eq!(align_of::<ElfHead32>(), 4);
    assert_eq!(align_of::<ElfHead64>(), 8);
    assert_eq!(align_of::<ProgHead32>(), 4);
    assert_eq!(align_of::<ProgHead64>(), 8);
    assert_eq!(align_of::<SectNonArchDep>(), 4);
    assert_eq!(align_of::<SectHead32>(), 4);
    assert_eq!(align_of::<SectHead64>(), 8);
    assert_eq!(align_of::<Sym32>(), 4);
    assert_eq!(align_of::<Sym64>(), 8);

    assert_eq!(align_of::<EIClass>(), 1);
    assert_eq!(align_of::<EIClassUnchecked>(), 1);
    assert_eq!(align_of::<EIData>(), 1);
    assert_eq!(align_of::<EIDataUnchecked>(), 1);
    assert_eq!(align_of::<EIOsAbi>(), 1);
    assert_eq!(align_of::<EIOsAbiUnchecked>(), 1);
    assert_eq!(align_of::<EMachine>(), 2);
    assert_eq!(align_of::<EMachineUnchecked>(), 2);
    assert_eq!(align_of::<PType>(), 4);
    assert_eq!(align_of::<PTypeUnchecked>(), 4);
    assert_eq!(align_of::<ShType>(), 4);
    assert_eq!(align_of::<ShTypeUnchecked>(), 4);

    assert_eq!(size_of::<EIdent>(), 16);
    assert_eq!(size_of::<ElfNonArchDep>(), 24);
    assert_eq!(size_of::<Elf32Offs>(), 12);
    assert_eq!(size_of::<Elf64Offs>(), 24);
    assert_eq!(size_of::<ElfNonArchDep2>(), 16);
    assert_eq!(size_of::<ElfHead32>(), 52);
    assert_eq!(size_of::<ElfHead64>(), 64);
    assert_eq!(size_of::<ProgHead32>(), 32);
    assert_eq!(size_of::<ProgHead64>(), 56);
    assert_eq!(size_of::<SectNonArchDep>(), 8);
    assert_eq!(size_of::<SectHead32>(), 40);
    assert_eq!(size_of::<SectHead64>(), 64);
    assert_eq!(size_of::<Sym32>(), 16);
    assert_eq!(size_of::<Sym64>(), 24);

    assert_eq!(size_of::<EIClass>(), 1);
    assert_eq!(size_of::<EIClassUnchecked>(), 1);
    assert_eq!(size_of::<EIData>(), 1);
    assert_eq!(size_of::<EIDataUnchecked>(), 1);
    assert_eq!(size_of::<EIOsAbi>(), 1);
    assert_eq!(size_of::<EIOsAbiUnchecked>(), 1);
    assert_eq!(size_of::<EMachine>(), 2);
    assert_eq!(size_of::<EMachineUnchecked>(), 2);
    assert_eq!(size_of::<PType>(), 4);
    assert_eq!(size_of::<PTypeUnchecked>(), 4);
    assert_eq!(size_of::<ShType>(), 4);
    assert_eq!(size_of::<ShTypeUnchecked>(), 4);
}

#[test]
fn miri_as_bytes_mut() {
    use rand::Fill;
    fn test<T: TransmuteSafe>() -> T {
        let mut t = T::default();
        let bytes = t.as_bytes_mut();
        for i in 0..bytes.len() {
            assert_eq!(bytes[i], 0);
        }
        bytes.try_fill(&mut rand::thread_rng()).unwrap();
        t
    }
    test::<ElfNonArchDep>();
    test::<Elf32Offs>();
    test::<Elf64Offs>();
    test::<ElfNonArchDep2>();
    test::<ProgHead32>();
    test::<ProgHead64>();
    test::<SectHead32>();
    test::<SectHead64>();
    test::<Sym32>();
    test::<Sym64>();
}

#[test]
fn miri_vec_as_bytes_mut() {
    use crate::utils::vec_as_bytes_mut;
    use rand::Fill;
    fn test<T: TransmuteSafe>() -> Vec<T> {
        let mut vec = Vec::new();
        let bytes = vec_as_bytes_mut(&mut vec, 3);
        for i in 0..bytes.len() {
            assert_eq!(bytes[i], 0);
        }
        bytes.try_fill(&mut rand::thread_rng()).unwrap();
        assert_eq!(vec.len(), 3);
        vec
    }
    test::<ElfNonArchDep>();
    test::<Elf32Offs>();
    test::<Elf64Offs>();
    test::<ElfNonArchDep2>();
    test::<ProgHead32>();
    test::<ProgHead64>();
    test::<SectHead32>();
    test::<SectHead64>();
    test::<Sym32>();
    test::<Sym64>();
}
