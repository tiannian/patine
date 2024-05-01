use core::marker::PhantomData;

use patine_core::{
    allocate,
    builtin::{keccak256, mstore},
    deallocate, AsNativeType, FromNativeType, U256,
};

use crate::{Error, Result};

use super::StorageBackend;

pub struct Array<T, B> {
    solt_prefix: U256,

    marker_t: PhantomData<T>,
    backend: B,
}

impl<T, B> Array<T, B>
where
    B: StorageBackend,
{
    fn compute_solt(&self, index: U256) -> U256 {
        let bytes = allocate(64);

        mstore(self.solt_prefix, bytes);
        mstore(index, &mut bytes[32..]);

        deallocate(64);

        keccak256(bytes).into()
    }

    pub fn len(&self) -> U256 {
        self.backend.load(self.solt_prefix)
    }

    pub fn clear(&mut self) {
        self.backend.store(self.solt_prefix, U256::from(0))
    }
}

impl<T, B> Array<T, B>
where
    T: AsNativeType + FromNativeType,
    B: StorageBackend,
{
    pub fn get(&self, index: U256) -> Option<T> {
        if index < self.len() {
            let solt = self.compute_solt(index);

            Some(self.backend.load(solt))
        } else {
            None
        }
    }

    pub fn set(&mut self, index: U256, value: T) -> Result<()> {
        if index < self.len() {
            let solt = self.compute_solt(index);

            self.backend.store(solt, value);

            Ok(())
        } else {
            Err(Error::ArrayOutOfBound)
        }
    }

    pub fn push(&mut self, value: T) {
        let index = self.len();

        let solt = self.compute_solt(index);

        self.backend.store(solt, value);
        self.backend.store(self.solt_prefix, index);
    }
}
