use core::fmt::Write;
use crate::{error::Error, os};

pub mod parse;
pub mod load;

pub fn e<T>(s: &str) -> Result<T, Error> {
    let _ = writeln!(os::STDERR, "{}", s);
    Err(Error::Elf)
}