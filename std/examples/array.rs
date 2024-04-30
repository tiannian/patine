use patine_core::{builtin::sstore, U256};
use patine_std::uint;

#[no_mangle]
pub fn _store_deployed(res: [U256; 8]) {
    // let res = [U256::default(); 8];

    sstore(uint!(0), res[0]);
    sstore(uint!(0), res[1]);
    sstore(uint!(0), res[2]);
    sstore(uint!(0), res[3]);
    sstore(uint!(0), res[4]);
    sstore(uint!(0), res[5]);
}
