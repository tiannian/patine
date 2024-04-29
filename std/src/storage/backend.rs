use patine_core::{builtin, AsNativeType, FromNativeType, U256};

pub trait StorageBackend {
    fn store(key: U256, value: impl AsNativeType);

    fn load<V>(key: U256) -> V
    where
        V: FromNativeType;
}

pub struct Storage {}

impl StorageBackend for Storage {
    fn load<V>(key: U256) -> V
    where
        V: FromNativeType,
    {
        builtin::sload(key)
    }

    fn store(key: U256, value: impl AsNativeType) {
        builtin::sstore(key, value)
    }
}

pub struct TransientStorage {}

impl StorageBackend for TransientStorage {
    fn store(key: U256, value: impl AsNativeType) {
        builtin::tstore(key, value)
    }

    fn load<V>(key: U256) -> V
    where
        V: FromNativeType,
    {
        builtin::tload(key)
    }
}
