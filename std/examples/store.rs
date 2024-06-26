#![no_std]

use patine_core::{
    allocate,
    builtin::{
        calldataload, caller, callvalue, datacopy, iszero, mstore, objectoffset, objectsize, ret,
        revert, sload, sstore,
    },
    AsNativeType, U256,
};
use patine_std::{require, selector, uint};

#[no_mangle]
pub extern "C" fn _store() {
    sstore(uint!(0), caller());

    let code = allocate(objectsize(_store_deployed));

    let offset = objectoffset(_store_deployed);

    datacopy(offset, code);

    ret(code)
}

fn retrieve() {
    let v: U256 = sload(uint!(0));

    let mut ret_arr = [0u8; 32];

    mstore(v, &mut ret_arr);
    ret(&ret_arr);
}

fn store() {
    let v: U256 = calldataload(4);

    sstore(uint!(0), v)
}

#[no_mangle]
pub extern "C" fn _store_deployed() {
    require(iszero(callvalue()));

    match selector().as_native_type() {
        0x2e64cec1 => retrieve(),
        0x6057361d => store(),
        _ => revert(&[]),
    }
}
