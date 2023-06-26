use casper_types::account::{AccountHash, Weight};

pub const ADD_ACCOUNT_WASM: &str = "add_account.wasm";
pub const REMOVE_ACCOUNT_WASM: &str = "remove_account.wasm";
pub const UPDATE_KEYS_WASM: &str = "update_associated_keys.wasm";
pub const UPDATE_THRESHOLDS_WASM: &str = "update_thresholds.wasm";

pub const USER_1_ACCOUNT: AccountHash = AccountHash::new([1u8; 32]);
pub const USER_2_ACCOUNT: AccountHash = AccountHash::new([2u8; 32]);
pub const USER_3_ACCOUNT: AccountHash = AccountHash::new([3u8; 32]);
pub const USER_4_ACCOUNT: AccountHash = AccountHash::new([4u8; 32]);

pub const EXPECTED_KEY_WEIGHT: Weight = Weight::new(3);
pub const DEPLOYMENT_WEIGHT: Weight = Weight::new(1);
pub const DEPLOYMENT_THRESHOLD: Weight = Weight::new(2);
pub const KEY_MGMT_THRESHOLD: Weight = Weight::new(3);
