use patine_core::{builtin, AsNativeType, FromNativeType, U256};

pub trait StorageBackend: Default {
    fn store(&mut self, key: U256, value: impl AsNativeType);

    fn load<V>(&self, key: U256) -> V
    where
        V: FromNativeType;
}

#[derive(Default)]
pub struct Storage {}

impl StorageBackend for Storage {
    fn load<V>(&self, key: U256) -> V
    where
        V: FromNativeType,
    {
        builtin::sload(key)
    }

    fn store(&mut self, key: U256, value: impl AsNativeType) {
        builtin::sstore(key, value)
    }
}

#[derive(Default)]
pub struct TransientStorage {}

impl StorageBackend for TransientStorage {
    fn store(&mut self, key: U256, value: impl AsNativeType) {
        builtin::tstore(key, value)
    }

    fn load<V>(&self, key: U256) -> V
    where
        V: FromNativeType,
    {
        builtin::tload(key)
    }
}
