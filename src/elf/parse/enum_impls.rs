use core::{fmt::{self, Debug, Formatter}};

use crate::{
    elf::parse::ffi_types::{EIClass, EIClassUnchecked, EIData, EIDataUnchecked, EIOsAbi, EIOsAbiUnchecked, EType, ETypeUnchecked, EMachine, EMachineUnchecked, PType, PTypeUnchecked, ShType, ShTypeUnchecked},
    utils::ToKnown,
};

impl ToKnown for EIClassUnchecked {
    type Known = EIClass;
    type Unknown = u8;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = self.unknown();
        if u == 0x01 || u == 0x02 {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl ToKnown for EIDataUnchecked {
    type Known = EIData;
    type Unknown = u8;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = self.unknown();
        if u == 0x01 || u == 0x02 {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl ToKnown for EIOsAbiUnchecked {
    type Known = EIOsAbi;
    type Unknown = u8;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = self.unknown();
        if u == 0x00 || u == 0x03 {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl ToKnown for ETypeUnchecked {
    type Known = EType;
    type Unknown = u16;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = unsafe { self.unknown };
        if (0x00..=0x04).contains(&u) {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl ToKnown for EMachineUnchecked {
    type Known = EMachine;
    type Unknown = u16;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = self.unknown();
        if u == 0x03 || u == 0x3E || u == 0xB7 {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl ToKnown for PTypeUnchecked {
    type Known = PType;
    type Unknown = u32;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = self.unknown();
        if (0x00..=0x07).contains(&u) || [0x6474e550, 0x6474e551, 0x6474e552].contains(&u) {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl ToKnown for ShTypeUnchecked {
    type Known = ShType;
    type Unknown = u32;

    fn known(&self) -> Result<Self::Known, Self::Unknown> {
        let u = self.unknown();
        if (0x00..=0x0B).contains(&u) || (0x0E..=0x13).contains(&u) || [0x6FFFFFFF, 0x6FFFFFFE, 0x6FFFFFFD, 0x6FFFFFF6].contains(&u) {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
    }
}

impl Default for EIClassUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Default for EIDataUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Default for EIOsAbiUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Default for ETypeUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Default for EMachineUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Default for PTypeUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Default for ShTypeUnchecked {
    fn default() -> Self {
        Self { unknown: 0 }
    }
}

impl Debug for EIClassUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownEIClass(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for EIDataUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownEIData(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for EIOsAbiUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownEIOsAbi(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for ETypeUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownEType(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for EMachineUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownEMachine(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for PTypeUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownPType(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for ShTypeUnchecked {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "UnknownShType(0x{:X?})", self.unknown())
        }
    }
}

impl PartialEq for EIClassUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}
impl PartialEq for EIDataUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}
impl PartialEq for EIOsAbiUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}

impl PartialEq for ETypeUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}

impl PartialEq for EMachineUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}

impl PartialEq for PTypeUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}

impl PartialEq for ShTypeUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unknown().eq(&other.unknown())
    }
}

#[test]
fn miri_enum() {
    use crate::elf::parse::TransmuteSafe;

    let mut ei_osabi = EIOsAbiUnchecked::default();
    for i in 0..0xFFu8 {
        let bytes = ei_osabi.as_bytes_mut();
        bytes.copy_from_slice(&i.to_le_bytes());
        assert_eq!(i, ei_osabi.unknown());
        match ei_osabi.known() {
            Ok(o) => assert_eq!(o as u8, i),
            Err(e) => assert_eq!(e, i),
        }
    }
    let mut e_type = ETypeUnchecked::default();
    for i in 0..0x01FFu16 {
        let bytes = e_type.as_bytes_mut();
        bytes.copy_from_slice(&i.to_le_bytes());
        assert_eq!(i, e_type.unknown());
        match e_type.known() {
            Ok(o) => assert_eq!(o as u16, i),
            Err(e) => assert_eq!(e, i),
        }
    }
    let mut e_machine = EMachineUnchecked::default();
    for i in 0..0x01FFu16 {
        let bytes = e_machine.as_bytes_mut();
        bytes.copy_from_slice(&i.to_le_bytes());
        assert_eq!(i, e_machine.unknown());
        match e_machine.known() {
            Ok(o) => assert_eq!(o as u16, i),
            Err(e) => assert_eq!(e, i),
        }
    }
    let mut p_type = PTypeUnchecked::default();
    for i in 0..0x01FFu32 {
        let bytes = p_type.as_bytes_mut();
        bytes.copy_from_slice(&i.to_le_bytes());
        assert_eq!(i, p_type.unknown());
        match p_type.known() {
            Ok(o) => assert_eq!(o as u32, i),
            Err(e) => assert_eq!(e, i),
        }
    }
    let mut sh_type = ShTypeUnchecked::default();
    for i in 0..0x01FFu32 {
        let bytes = sh_type.as_bytes_mut();
        bytes.copy_from_slice(&i.to_le_bytes());
        assert_eq!(i, sh_type.unknown());
        match sh_type.known() {
            Ok(o) => assert_eq!(o as u32, i),
            Err(e) => assert_eq!(e, i),
        }
    }
}
