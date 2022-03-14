use core::{mem::{size_of, align_of}, slice};

use crate::error::Error;

#[macro_export]
macro_rules! dbg {
    ($EXP: expr) => {
        {
            use core::fmt::Write;
            let _ = writeln!(crate::os::STDERR, "{:?}", $EXP);
        }
    };
}

pub trait ToKnown: TransmuteSafe {
    type Known;
    type Unknown;
    fn known(&self) -> Result<Self::Known, Self::Unknown>;
    fn unknown(&self) -> Self::Unknown;
}

pub unsafe trait TransmuteSafe: Default + Clone {
    fn from_buf(buf: &[u8]) -> Result<(&Self, &[u8]), Error> {
        if buf.len() < size_of::<Self>() {
            return Err(Error::Transmute);
        }
        if buf.as_ptr() as usize % align_of::<Self>() != 0 {
            return Err(Error::Transmute);
        }
        let tail = &buf[size_of::<Self>()..];
        let me = unsafe { &*(buf.as_ptr() as *const Self) };
        Ok((me, tail))
    }

    fn slice_from_buf(buf: &[u8], n: usize) -> Result<(&[Self], &[u8]), Error> {
        if buf.len() < n * size_of::<Self>() {
            return Err(Error::Transmute);
        }
        if buf.as_ptr() as usize % align_of::<Self>() != 0 {
            return Err(Error::Transmute);
        }
        let tail = &buf[n * size_of::<Self>()..];
        let us: &[Self] = unsafe { slice::from_raw_parts(buf.as_ptr() as *const Self, n) };
        Ok((us, tail))
    }
    /*
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        // This unsafe is sound because:
        // - Self is TransmuteSafe
        // - TransmuteSafe is an unsafe trait that guarantees that Self allows any byte pattern
        // - [u8] has alignment of 1, which is always less or equal than Self's alignment
        // - the size of [u8] is set to equal the size of Self in bytes
        // - The mutable access to the bytes of Self is constrained by the lifetime of &mut self
        // - Accepting &mut Self as an argument guarantees that its bytes are already initialized
        unsafe { from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    } */
}

/*
pub fn _as_bytes_mut<T: TransmuteSafe>(vec: &mut Vec<T>, n: usize) -> &mut [u8] {
    vec.clear();
    vec.resize(n, T::default());
    // This unsafe is sound because:
    // - vec is reserved to have a buffer that is large enough to fit the all Ts
    // - The buffer is filled with T::default() so all the bytes are initialized
    // - Self is TransmuteSafe
    // - TransmuteSafe is an unsafe trait that guarantees that Self allows any byte pattern
    // - [u8] has alignment of 1, which is always less or equal than Self's alignment
    // - The mutable access to the bytes of Self is constrained by the lifetime of &mut self
    unsafe { from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, n * size_of::<T>()) }
}
*/