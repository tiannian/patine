use crate::ffi::{self, EntryFunction};

#[inline]
pub fn datacopy(f: usize, target: &mut [u8]) {
    unsafe { ffi::__yul_datacopy(target.as_mut_ptr(), f, target.len()) }
}

#[inline]
pub fn objectsize(f: EntryFunction) -> usize {
    unsafe { ffi::__yul_objectsize(f) }
}

#[inline]
pub fn objectoffset(f: EntryFunction) -> usize {
    unsafe { ffi::__yul_objectoffset(f) }
}
