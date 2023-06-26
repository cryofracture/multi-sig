#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

use casper_contract::contract_api::{account, runtime};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::account::{AccountHash, Weight};

const RUNTIME_ARG_NEW_ASSOCIATED_KEY: &str = "new_key";
const RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT: &str = "weight";

#[no_mangle]
pub extern "C" fn call() {
    let new_associated_key: AccountHash = runtime::get_named_arg(RUNTIME_ARG_NEW_ASSOCIATED_KEY);
    let new_key_weight: u8 = runtime::get_named_arg(RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT);

    account::add_associated_key(new_associated_key, Weight::new(new_key_weight)).unwrap_or_revert();
}
