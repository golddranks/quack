use core::{fmt, str::Utf8Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Open(i32),
    Write(i32),
    Read(i32),
    Fstat(i32),
    Fmt(fmt::Error),
    Mmap(i32),
    Elf,
    Cli,
    Utf8Error,
    Transmute,
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Error {
        Error::Fmt(e)
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> fmt::Error {
        fmt::Error
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Error {
        Error::Utf8Error
    }
}

impl Error {
    pub fn to_ret(&self) -> u8 {
        match self {
            Error::Open(errno) => 0*16 + (errno % 16) as u8,
            Error::Write(errno) => 1*16 + (errno % 16) as u8,
            Error::Read(errno) => 2*16 + (errno % 16) as u8,
            Error::Fstat(errno) => 3*16 + (errno % 16) as u8,
            Error::Fmt(_) => 4*16,
            Error::Elf => 5*16,
            Error::Cli => 6*16,
            Error::Utf8Error => 7*16,
            Error::Transmute => 8*16,
            Error::Mmap(errno) => 9*16 + (errno % 16) as u8,
        }
    }
}

