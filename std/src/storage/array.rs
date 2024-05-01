use core::marker::PhantomData;

use patine_core::{
    allocate,
    builtin::{keccak256, mstore, sload, sstore},
    deallocate, AsNativeType, FromNativeType, U256,
};

use crate::{Error, Result};

use super::StorageBackend;

pub struct Array<T, B> {
    solt_prefix: U256,

    marker_t: PhantomData<T>,
    marker_b: PhantomData<B>,
}

impl<T, B> Array<T, B>
where
    T: AsNativeType + FromNativeType,
    B: StorageBackend,
{
    fn compute_solt(&self, index: U256) -> U256 {
        let bytes = allocate(64);

        mstore(self.solt_prefix, bytes);
        mstore(index, &mut bytes[32..]);

        deallocate(64);

        keccak256(bytes).into()
    }

    pub fn get(&self, index: U256) -> Option<T> {
        if index < self.length() {
            let solt = self.compute_solt(index);

            Some(sload(solt))
        } else {
            None
        }
    }

    pub fn set(&mut self, index: U256, value: T) -> Result<()> {
        if index < self.length() {
            let solt = self.compute_solt(index);

            sstore(solt, value);

            Ok(())
        } else {
            Err(Error::ArrayOutOfBound)
        }
    }

    pub fn length(&self) -> U256 {
        sload(self.solt_prefix)
    }

    pub fn push(&mut self, value: T) {
        let index = self.length();

        let solt = self.compute_solt(index);

        sstore(solt, value);
        sstore(self.solt_prefix, index);
    }

    pub fn clear(&mut self) {
        sstore(self.solt_prefix, U256::from(0))
    }
}
