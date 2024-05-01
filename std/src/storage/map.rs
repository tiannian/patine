use core::marker::PhantomData;

use patine_core::{
    allocate,
    builtin::{keccak256, mstore, sload},
    deallocate, AsNativeType, FromNativeType, U256,
};

use super::StorageBackend;

pub struct Map<K, V, B> {
    solt_prefix: U256,

    marker_k: PhantomData<K>,
    marker_v: PhantomData<V>,
    marker_b: PhantomData<B>,
}

impl<K, V, B> Map<K, V, B>
where
    K: AsNativeType + FromNativeType,
    V: AsNativeType + FromNativeType,
    B: StorageBackend,
{
    fn compute_solt(&self, index: K) -> U256 {
        let bytes = allocate(64);

        mstore(self.solt_prefix, bytes);
        mstore(index, &mut bytes[32..]);

        deallocate(64);

        keccak256(bytes).into()
    }

    pub fn get(&self, key: K) -> V {
        let solt = self.compute_solt(key);

        sload(solt)
    }
}
