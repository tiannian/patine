use core::marker::PhantomData;

use patine_core::{AsNativeType, FromNativeType, U256};

use super::StorageBackend;

pub struct Value<V, B> {
    solt: U256,
    marker_v: PhantomData<V>,
    marker_b: PhantomData<B>,
}

impl<V, B> Value<V, B>
where
    V: AsNativeType + FromNativeType,
    B: StorageBackend,
{
    pub fn set(&mut self, value: V) {
        B::store(self.solt, value)
    }

    pub fn get(&self) -> V {
        B::load(self.solt)
    }
}
