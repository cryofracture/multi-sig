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
    use casper_types::account::Weight;
    use std::path::PathBuf;

    #[allow(unused_variables)]
    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    pub const ACCOUNT_USER_1: [u8; 32] = [1u8; 32];
    pub const ACCOUNT_USER_2: [u8; 32] = [2u8; 32];
    pub const ACCOUNT_USER_3: [u8; 32] = [3u8; 32];
    pub const ACCOUNT_USER_4: [u8; 32] = [4u8; 32];
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
    const KEY_MANAGEMENT_WEIGHT: u8 = 3;
    const DEPLOYMENT_WEIGHT: u8 = 2;
    const STD_KEY_WEIGHT: u8 = 1;
    const PRIMARY_KEY_NEW_WEIGHT: u8 = 0;
    const KEY_WEIGHT: &str = "weight";
    const EXPECTED_KEY_WEIGHT: Weight = Weight::new(3);// = Weight::new(new_weight);

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
                RUNTIME_ARG_NEW_KEY_WEIGHT => KEY_MANAGEMENT_WEIGHT,
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
            .unwrap()
            .associated_keys()
            .get(&DEFAULT_ACCOUNT_ADDR)
            .expect("Should have own account");
        // .contains_key(&DEFAULT_ACCOUNT_ADDR)
        // .get(KEY_WEIGHT);
        // let actual_weight = account.associated_keys().get(&DEFAULT_ACCOUNT_ADDR);
        // let expected_weight: Weight::new<KEY_MANAGEMENT_WEIGHT>;
        // dbg!(actual_weight);

        // assert_eq!(account, Some(&EXPECTED_KEY_WEIGHT));
        // let account_weight = account as u8;
        // assert!(account_weight == KEY_MANAGEMENT_WEIGHT);
    }

    // #[test]
    // fn should_add_new_account_to_primary() {
    //     let mut builder = InMemoryWasmTestBuilder::default();
    //     builder.run_genesis(&*PRODUCTION_RUN_GENESIS_REQUEST).commit();
    //
    //
    //     // Install the contract.
    //     let contract_installation_request = ExecuteRequestBuilder::standard(
    //         *DEFAULT_ACCOUNT_ADDR,
    //         UPDATE_KEYS_WASM,
    //         runtime_args! {
    //             RUNTIME_ARG_ASSOCIATED_KEY => *ACCOUNT_USER_1,
    //             RUNTIME_ARG_NEW_KEY_WEIGHT => 3,
    //         },
    //     )
    //         .build();
    //
    //     builder
    //         .exec(contract_installation_request)
    //         .expect_success()
    //         .commit();
    //
    //     // Prepare assertions.
    //     let account = builder
    //         .get_account(*DEFAULT_ACCOUNT_ADDR)
    //         .expect("should have account");
    //     let actual_weight = account.associated_keys().get(&ACCOUNT_USER_1).expect_success();
    // }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
