use std::fmt::Debug;



#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EIdent {
    pub(super) ei_mag: [u8; 4],
    pub(super) ei_class: u8,
    pub(super) ei_data: u8,
    pub(super) ei_version: u8,
    pub(super) ei_osabi: EIOsAbiUnchecked,
    pub(super) ei_abiversion: u8,
    pub(super) ei_pad: [u8; 7],
}

#[derive(Copy, Clone)]
pub union EIOsAbiUnchecked {
    pub(super) unchecked: u8,
    pub(super) checked: EIOsAbi,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EIOsAbi {
    SystemV = 0x00,
    HpUx = 0x01,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ElfNonArchDep {
    pub(super) e_ident: EIdent,
    pub(super) e_type: u16,
    pub(super) e_machine: u16,
    pub(super) e_version: u32,
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
    pub(super) p_type: u32,
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
    pub(super) p_type: u32,
    pub(super) p_offset: u32,
    pub(super) p_vaddr: u32,
    pub(super) p_paddr: u32,
    pub(super) p_filesz: u32,
    pub(super) p_memsz: u32,
    pub(super) p_flag: u32,
    pub(super) p_align: u32,
}

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SectNonArchDep {
    pub(super) sh_name: u32,
    pub(super) sh_type: u32,
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
use crate::elf::TransmuteSafe;


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
    use crate::elf::vec_as_bytes_mut;
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

#[test]
fn miri_enum() {
    unsafe impl TransmuteSafe for EIOsAbiUnchecked {}
    let mut ei_osabi = EIOsAbiUnchecked::default();
    let bytes = ei_osabi.as_bytes_mut();
    bytes[0] = 0xFF;
    eprintln!("{:?}", ei_osabi);
}
