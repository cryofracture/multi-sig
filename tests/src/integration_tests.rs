#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder,
        ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT, PRODUCTION_RUN_GENESIS_REQUEST,
    };
    use casper_execution_engine::core::{engine_state::Error as EngineStateError, execution};
    use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
    use casper_types::ContractHash;
    use casper_types::{api_error::ApiError, Key};
    use casper_types::{runtime_args, RuntimeArgs};
    use std::path::PathBuf;

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

    #[test]
    fn should_update_primary_key_weight() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST).commit();

        let session_code = PathBuf::from(UPDATE_KEYS_WASM);
        let session_args = runtime_args! {
            RUNTIME_ARG_ASSOCIATED_KEY => DEFAULT_ACCOUNT_ADDR,
            RUNTIME_ARG_NEW_KEY_WEIGHT => 3,
        };

        let deploy_item = DeployItemBuilder::new()
            .with_empty_payment_bytes(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

        // prepare assertions.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");
        let actual_weight = account.associated_keys().get(&DEFAULT_ACCOUNT_ADDR);
        assert!(account.is_err());

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();
    }

    // #[test]
    // fn should_add_new_account_to_primary() {
    //     let mut builder = InMemoryWasmTestBuilder::default();
    //     builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST).commit();
    //
    //     let session_code = PathBuf::from(ADD_ACCOUNT_WASM);
    //     let session_args = runtime_args! {
    //         RUNTIME_ARG_ASSOCIATED_KEY => ACCOUNT_USER_1,
    //         RUNTIME_ARG_NEW_KEY_WEIGHT => 1,
    //     };
    //
    //     let deploy_item = DeployItemBuilder::new()
    //         .with_empty_payment_bytes(runtime_args! {
    //             ARG_AMOUNT => *DEFAULT_PAYMENT
    //         })
    //         .with_session_code(session_code, session_args)
    //         .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
    //         .with_address(*DEFAULT_ACCOUNT_ADDR)
    //         .build();
    //
    //     let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();
    //
    //     // prepare assertions.
    //     let result_of_query = builder.query(
    //         None,
    //         Key::Account(*DEFAULT_ACCOUNT_ADDR),
    //         &[CONTRACT_QUESTION_KEY.to_string()],
    //     );
    //     assert!(result_of_query.is_err());
    //
    //     // deploy the contract.
    //     builder.exec(execute_request).commit().expect_success();
    // }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
