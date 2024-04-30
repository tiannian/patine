use patine_core::{builtin, Address, Bytes32, U256};

pub struct Block {}

impl Block {
    #[inline]
    pub fn chain_id() -> U256 {
        builtin::chainid()
    }

    #[inline]
    pub fn base_fee() -> U256 {
        builtin::basefee()
    }

    #[inline]
    pub fn blob_base_fee() -> U256 {
        builtin::blobbasefee()
    }

    #[inline]
    pub fn block_hash(number: U256) -> Bytes32 {
        builtin::blockhash(number)
    }

    #[inline]
    pub fn coinbase() -> Address {
        builtin::coinbase()
    }

    #[inline]
    pub fn timestamp() -> U256 {
        builtin::timestamp()
    }

    #[inline]
    pub fn number() -> U256 {
        builtin::number()
    }

    #[inline]
    pub fn difficulty() -> U256 {
        builtin::difficulty()
    }

    #[inline]
    pub fn prevrandao() -> Bytes32 {
        builtin::prevrandao()
    }

    #[inline]
    pub fn gaslimit() -> U256 {
        builtin::gaslimit()
    }
}
