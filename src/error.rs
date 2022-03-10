use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Open(i32),
    Write(i32),
    Read(i32),
    Fmt(fmt::Error),
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
