use crate::{ffi, AsNativeType, FromNativeType, U256};

#[inline]
pub fn sload<T>(p: U256) -> T
where
    T: FromNativeType,
{
    T::from_native_type(unsafe { ffi::__yul_sload(p.0) })
}

#[inline]
pub fn sstore<T>(p: U256, v: T)
where
    T: AsNativeType,
{
    unsafe { ffi::__yul_sstore(p.0, v.as_native_type()) }
}

#[inline]
pub fn tload<T>(p: U256) -> T
where
    T: FromNativeType,
{
    T::from_native_type(unsafe { ffi::__yul_tload(p.0) })
}

#[inline]
pub fn tstore<T>(p: U256, v: T)
where
    T: AsNativeType,
{
    unsafe { ffi::__yul_tstore(p.0, v.as_native_type()) }
}
