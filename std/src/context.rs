use patine_core::{builtin, Address, U256};

use crate::data::{Calldata, Data};

pub struct Context {}

impl Context {
    #[inline]
    pub fn gas(&self) -> U256 {
        builtin::gas()
    }

    #[inline]
    pub fn caller(&self) -> Address {
        builtin::caller()
    }

    #[inline]
    pub fn value(&self) -> U256 {
        builtin::callvalue()
    }

    #[inline]
    pub fn calldata(&self) -> impl Data {
        Calldata {}
    }

    // pub fn return_data(&self) -> impl Data {}
}
