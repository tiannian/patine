use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

use crate::{ffi::*, AsNativeType, Bytes32, FromNativeType, Integer};

#[repr(C)]
#[repr(align(32))]
#[derive(Default, Clone, Copy)]
pub struct U256(pub(crate) NativeType);

impl AsNativeType for U256 {
    fn as_native_type(&self) -> NativeType {
        self.0
    }
}

impl FromNativeType for U256 {
    fn from_native_type(x: NativeType) -> Self {
        Self(x)
    }
}

impl Integer for U256 {}

// macro_rules! defined_uint {
//     ($s:ident) => {
//         #[repr(C)]
//         #[repr(align(32))]
//         #[derive(Default, Clone, Copy)]
//         pub struct $s(pub(crate) NativeType);
//
//         impl $s {
//             #[inline]
//             pub fn from_raw(x0: u64, x1: u64, x2: u64, x3: u64) -> Self {
//                 Self(unsafe { __yul__ext_literal(x0, x1, x2, x3) })
//             }
//         }
//
//         impl NativeTypeTrans for $s {
//             fn as_native_type(&self) -> NativeType {
//                 self.0
//             }
//
//             fn from_native_type(x: NativeType) -> Self {
//                 Self(x)
//             }
//         }
//
//         impl From<Bytes32> for $s {
//             #[inline]
//             fn from(value: Bytes32) -> Self {
//                 Self(value.0)
//             }
//         }
//
//         defined_uint_ops!($s, Add, add, __yul_add);
//         defined_uint_ops!($s, Sub, sub, __yul_sub);
//         defined_uint_ops!($s, Mul, mul, __yul_mul);
//         defined_uint_ops!($s, Div, div, __yul_div);
//         defined_uint_ops!($s, Rem, rem, __yul_mod);
//
//         defined_uint_ops!($s, Shr, shr, __yul_shr);
//         defined_uint_ops!($s, Shl, shl, __yul_shl);
//
//         defined_uint_ops!($s, BitAnd, bitand, __yul_and);
//         defined_uint_ops!($s, BitOr, bitor, __yul_or);
//         defined_uint_ops!($s, BitXor, bitxor, __yul_xor);
//
//         impl Not for $s {
//             type Output = Self;
//
//             #[inline]
//             fn not(self) -> Self::Output {
//                 Self(!self.0)
//             }
//         }
//
//         impl PartialEq for $s {
//             #[inline]
//             fn eq(&self, other: &Self) -> bool {
//                 self.0.eq(&other.0)
//             }
//         }
//
//         impl Eq for $s {}
//
//         impl PartialOrd for $s {
//             #[inline]
//             fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
//                 Some(self.cmp(other))
//             }
//         }
//
//         impl Ord for $s {
//             #[inline]
//             fn cmp(&self, other: &Self) -> core::cmp::Ordering {
//                 self.0.cmp(&other.0)
//             }
//         }
//     };
// }
//
// defined_uint!(U256);
// defined_uint!(U128);
// defined_uint!(U64);
// defined_uint!(U32);
// defined_uint!(U16);
// defined_uint!(U8);
