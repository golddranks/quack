use std::fmt::Debug;

use crate::{
    e,
    elf::{
        ffi_types::{EIOsAbi, EIOsAbiUnchecked},
        ToChecked,
    },
    Error,
};

impl ToChecked for EIOsAbiUnchecked {
    type Checked = EIOsAbi;
    type Unchecked = u8;

    fn check(&self) -> Result<Self::Checked, Error> {
        if self.unchecked() <= 0x12 {
            Ok(unsafe { self.checked })
        } else {
            e("invalid value for e_ident.EI_OSABI")
        }
    }

    fn unchecked(&self) -> Self::Unchecked {
        unsafe { self.unchecked }
    }
}

impl Default for EIOsAbiUnchecked {
    fn default() -> Self {
        Self { unchecked: 0 }
    }
}

impl Debug for EIOsAbiUnchecked {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.unchecked().fmt(f)
    }
}

impl PartialEq for EIOsAbiUnchecked {
    fn eq(&self, other: &Self) -> bool {
        self.unchecked().eq(&other.unchecked())
    }
}
