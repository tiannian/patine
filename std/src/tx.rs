use patine_core::{builtin, Address, Bytes32, U256};

pub struct Transaction {}

impl Transaction {
    #[inline]
    pub fn origin() -> Address {
        builtin::origin()
    }

    #[inline]
    pub fn gasprice() -> U256 {
        builtin::gas()
    }

    #[inline]
    pub fn blob_hash(index: U256) -> Bytes32 {
        builtin::blobhash(index)
    }
}
