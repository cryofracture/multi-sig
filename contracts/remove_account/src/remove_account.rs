#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

use casper_contract::contract_api::{account, runtime};
use casper_types::account::AccountHash;
use remove_account::constants::RUNTIME_ARG_REMOVE_ASSOCIATED_KEY;

#[no_mangle]
pub extern "C" fn call() {
    let key_to_remove: AccountHash = runtime::get_named_arg(RUNTIME_ARG_REMOVE_ASSOCIATED_KEY);

    let _ret = account::remove_associated_key(key_to_remove);
}
