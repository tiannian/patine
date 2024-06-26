use crate::ffi::NativeType;

pub trait AsNativeType {
    fn as_native_type(&self) -> NativeType;
}

pub trait FromNativeType {
    fn from_native_type(x: NativeType) -> Self;
}

pub trait Integer {}

pub trait FixedBytes {}

pub trait UnsafeFromLiteral {
    fn from_literal(a: NativeType, b: NativeType, c: NativeType, d: NativeType) -> Self;
}
