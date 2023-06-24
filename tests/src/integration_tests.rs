#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder,
        ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT, PRODUCTION_RUN_GENESIS_REQUEST,
        transfer::{create_accounts_request},
    };
    use casper_execution_engine::core::{engine_state::Error as EngineStateError, execution};
    use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
    use casper_types::ContractHash;
    use casper_types::{api_error::ApiError, Key};
    use casper_types::{runtime_args, RuntimeArgs};
    use casper_types::account::{Weight, AccountHash};
    use std::path::PathBuf;

    #[allow(unused_variables)]
    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    pub const USER_1_ACCOUNT: [u8; 32] = [1u8; 32];
    pub const USER_1_ACCOUNT_HASH: AccountHash = AccountHash::new(USER_1_ACCOUNT);
    pub const USER_2_ACCOUNT: [u8; 32] = [2u8; 32];
    pub const USER_2_ACCOUNT_HASH: AccountHash = AccountHash::new(USER_2_ACCOUNT);
    pub const USER_3_ACCOUNT: [u8; 32] = [3u8; 32];
    pub const USER_3_ACCOUNT_HASH: AccountHash = AccountHash::new(USER_3_ACCOUNT);
    pub const USER_4_ACCOUNT: [u8; 32] = [4u8; 32];
    pub const USER_4_ACCOUNT_HASH: AccountHash = AccountHash::new(USER_4_ACCOUNT);
    // Define `KEY` constant to match that in the contract.
    const RUNTIME_ARG_NEW_ASSOCIATED_KEY: &str = "new_key";
    const RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT: &str = "weight";
    const RUNTIME_ARG_KEY_NAME: &str = "key_name";
    const RUNTIME_ARG_NAME: &str = "message";
    const RUNTIME_ARG_REMOVE_ASSOCIATED_KEY: &str = "remove_key";
    const RUNTIME_ARG_ASSOCIATED_KEY: &str = "associated_key";
    const RUNTIME_ARG_NEW_KEY_WEIGHT: &str = "new_weight";
    const RUNTIME_ARG_NEW_DEPLOYMENT_THRESHOLD: &str = "deployment_threshold";
    const RUNTIME_ARG_NEW_KEY_MANAGEMENT_THRESHOLD: &str = "key_management_threshold";
    const ADD_ACCOUNT_WASM: &str = "add_account.wasm";
    const HELLO_WORLD_WASM: &str = "hello_world.wasm";
    const REMOVE_ACCOUNT_WASM: &str = "remove_account.wasm";
    const UPDATE_KEYS_WASM: &str = "update_associated_keys.wasm";
    const UPDATE_THRESHOLDS_WASM: &str = "update_thresholds.wasm";
    const PRIMARY_KEY_NEW_WEIGHT: Weight = Weight::new(0);
    const EXPECTED_KEY_WEIGHT: Weight = Weight::new(3);
    const DEPLOYMENT_WEIGHT: Weight = Weight::new(1);
    const DEPLOYMENT_THRESHOLD: Weight = Weight::new(2);
    const KEY_MGMT_THRESHOLD: Weight = Weight::new(3);
    const ACTION_THRESHOLD_KEY_MGMT: &str = "key_management";
    const ACTION_THRESHOLD_DEPLOYMENT: &str = "deployment";
    const KEY_NAME: &str = "test-key";
    const RUNTIME_ARG_MESSAGE: &str = "message";
    const MESSAGE_TO_STORE: &str = "Hello, world!";

    #[test]
    fn should_update_primary_key_weight() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST).commit();


        // Install the contract.
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            UPDATE_KEYS_WASM,
            runtime_args! {
                RUNTIME_ARG_ASSOCIATED_KEY => *DEFAULT_ACCOUNT_ADDR,
                RUNTIME_ARG_NEW_KEY_WEIGHT => EXPECTED_KEY_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let actual_weight = account
            .associated_keys()
            .get(&DEFAULT_ACCOUNT_ADDR);
        assert_eq!(actual_weight, Some(&EXPECTED_KEY_WEIGHT));
    }

    #[test]
    fn should_add_new_accounts_to_primary() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST).commit();

        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_1_ACCOUNT_HASH,
                RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT => DEPLOYMENT_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let actual_weight = account
            .associated_keys()
            .get(&USER_1_ACCOUNT_HASH);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));

        // Add User Account 2 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_2_ACCOUNT_HASH,
                RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT => DEPLOYMENT_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let actual_weight = account
            .associated_keys()
            .get(&USER_2_ACCOUNT_HASH);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));

        // Add User Account 3 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_3_ACCOUNT_HASH,
                RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT => DEPLOYMENT_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let actual_weight = account
            .associated_keys()
            .get(&USER_3_ACCOUNT_HASH);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));

        // Add User Account 4 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_4_ACCOUNT_HASH,
                RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT => DEPLOYMENT_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let actual_weight = account
            .associated_keys()
            .get(&USER_4_ACCOUNT_HASH);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));
    }

    #[test]
    fn should_update_primary_key_weight_and_thresholds() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST).commit();


        // Install the contract.
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            UPDATE_KEYS_WASM,
            runtime_args! {
                RUNTIME_ARG_ASSOCIATED_KEY => *DEFAULT_ACCOUNT_ADDR,
                RUNTIME_ARG_NEW_KEY_WEIGHT => EXPECTED_KEY_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let actual_weight = account
            .associated_keys()
            .get(&DEFAULT_ACCOUNT_ADDR);
        assert_eq!(actual_weight, Some(&EXPECTED_KEY_WEIGHT));

        // Install the contract.
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            UPDATE_THRESHOLDS_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_DEPLOYMENT_THRESHOLD => DEPLOYMENT_THRESHOLD,
                RUNTIME_ARG_NEW_KEY_MANAGEMENT_THRESHOLD => KEY_MGMT_THRESHOLD,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let key_mgmt_threshold = account
            .action_thresholds()
            .key_management();
        // dbg!(key_mgmt_threshold);
        let deployment_threshold = account
            .action_thresholds()
            .deployment();
        assert_eq!(Some(key_mgmt_threshold), Some(&KEY_MGMT_THRESHOLD));
        assert_eq!(Some(deployment_threshold), Some(&DEPLOYMENT_THRESHOLD));
    }

    #[test]
    fn should_add_two_keys_and_remove_one() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST).commit();


        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_1_ACCOUNT_HASH,
                RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT => DEPLOYMENT_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_2_ACCOUNT_HASH,
                RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT => DEPLOYMENT_WEIGHT,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            REMOVE_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_REMOVE_ASSOCIATED_KEY => USER_1_ACCOUNT_HASH,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("Should be an account.");
        let missing_account = account
            .associated_keys()
            .get(&USER_1_ACCOUNT_HASH);

        let existing_account = account
            .associated_keys()
            .get(&USER_2_ACCOUNT_HASH);

        assert_eq!(existing_account, Some(&DEPLOYMENT_WEIGHT));
    }

    #[test]
    fn should_store_hello_world() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST).commit();


        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            HELLO_WORLD_WASM,
            runtime_args! {
                RUNTIME_ARG_MESSAGE => MESSAGE_TO_STORE,
                RUNTIME_ARG_KEY_NAME => KEY_NAME,
            },
        )
            .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Prepare assertions.
        // let account = builder
        //     .get_account(*DEFAULT_ACCOUNT_ADDR)
        //     .expect("Should be an account.");
        // let hello_world_string = account
        //     .named_keys()
        //     .get(&KEY_NAME);

        // make assertions
        let hello_world_string = builder
            .query(None, Key::Account(*DEFAULT_ACCOUNT_ADDR), &[KEY_NAME.to_string()])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        assert_eq!(hello_world_string, *MESSAGE_TO_STORE);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
