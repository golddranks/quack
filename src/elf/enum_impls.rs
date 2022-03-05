use std::fmt::Debug;

use crate::{
    elf::{
        ffi_types::{EIOsAbi, EIOsAbiUnchecked, EType, ETypeUnchecked, EMachine, EMachineUnchecked, PType, PTypeUnchecked, ShType, ShTypeUnchecked},
        ToKnown,
    },
};

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
        if (0x00..0x07).contains(&u) {
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
        if (0x00..0x0B).contains(&u) || (0x0E..=0x13).contains(&u) {
            Ok(unsafe { self.known })
        } else {
            Err(u)
        }
    }

    fn unknown(&self) -> Self::Unknown {
        unsafe { self.unknown }
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

impl Debug for EIOsAbiUnchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "Unknown(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for ETypeUnchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "Unknown(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for EMachineUnchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "Unknown(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for PTypeUnchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "Unknown(0x{:X?})", self.unknown())
        }
    }
}

impl Debug for ShTypeUnchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(t) = self.known() {
            t.fmt(f)
        } else {
            write!(f, "Unknown(0x{:X?})", self.unknown())
        }
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
