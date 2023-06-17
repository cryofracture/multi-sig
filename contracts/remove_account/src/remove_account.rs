#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::contract_api::{account, runtime};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::account::{AccountHash, ActionType, Weight};

const RUNTIME_ARG_REMOVE_ASSOCIATED_KEY: &str = "remove_key";


#[no_mangle]
pub extern "C" fn call() {
    let key_to_remove: AccountHash = runtime::get_named_arg(RUNTIME_ARG_REMOVE_ASSOCIATED_KEY);

    account::remove_associated_key(key_to_remove);
}