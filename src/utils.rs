use std::{mem::size_of, slice::from_raw_parts_mut};

pub trait ToKnown: TransmuteSafe {
    type Known;
    type Unknown;
    fn known(&self) -> Result<Self::Known, Self::Unknown>;
    fn unknown(&self) -> Self::Unknown;
}

pub unsafe trait TransmuteSafe: Default + Clone {
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        // This unsafe is sound because:
        // - Self is TransmuteSafe
        // - TransmuteSafe is an unsafe trait that guarantees that Self allows any byte pattern
        // - [u8] has alignment of 1, which is always less or equal than Self's alignment
        // - the size of [u8] is set to equal the size of Self in bytes
        // - The mutable access to the bytes of Self is constrained by the lifetime of &mut self
        // - Accepting &mut Self as an argument guarantees that its bytes are already initialized
        unsafe { from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    }
}

pub fn vec_as_bytes_mut<T: TransmuteSafe>(vec: &mut Vec<T>, n: usize) -> &mut [u8] {
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
