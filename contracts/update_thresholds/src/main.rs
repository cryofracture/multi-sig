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


// const RUNTIME_ARG_NEW_DEPLOYMENT_THRESHOLD: &str = "deployment_threshold";
// const RUNTIME_ARG_NEW_KEY_MANAGEMENT_THRESHOLD: &str = "key_management_threshold";


#[no_mangle]
pub extern "C" fn call() {

    // let deployment_threshold: u8 = runtime::get_named_arg(RUNTIME_ARG_NEW_DEPLOYMENT_THRESHOLD);
    // let key_mgmt_threshold: u8 = runtime::get_named_arg(RUNTIME_ARG_NEW_KEY_MANAGEMENT_THRESHOLD);

    account::set_action_threshold(ActionType::KeyManagement, Weight::new(3)).unwrap_or_revert();
    account::set_action_threshold(ActionType::Deployment, Weight::new(2)).unwrap_or_revert();

}