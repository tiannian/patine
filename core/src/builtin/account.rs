use crate::{define_one, ffi, Address, AsNativeType, Bytes32, U256};

define_one!(balance, Address, U256, __yul_balance);

#[inline]
pub fn extcodesize(a: Address) -> usize {
    unsafe { ffi::__yul_extcodesize(a.as_native_type()) }
}

#[inline]
pub fn extcodecopy(a: Address, t: &mut [u8], f: usize) {
    unsafe { ffi::__yul_extcodecopy(a.as_native_type(), t.as_mut_ptr(), f, t.len()) }
}

#[inline]
pub fn extcodehash(a: Address) -> Bytes32 {
    Bytes32(unsafe { ffi::__yul_extcodehash(a.as_native_type()) })
}
