#[cfg(test)]
mod tests {
    use add_account::constants::{
        RUNTIME_ARG_NEW_ASSOCIATED_KEY, RUNTIME_ARG_NEW_ASSOCIATED_KEY_WEIGHT,
    };
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        PRODUCTION_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, RuntimeArgs};
    use remove_account::constants::RUNTIME_ARG_REMOVE_ASSOCIATED_KEY;
    use tests::constants::{
        ADD_ACCOUNT_WASM, DEPLOYMENT_THRESHOLD, DEPLOYMENT_WEIGHT, EXPECTED_KEY_WEIGHT,
        KEY_MGMT_THRESHOLD, REMOVE_ACCOUNT_WASM, UPDATE_KEYS_WASM, UPDATE_THRESHOLDS_WASM,
        USER_1_ACCOUNT, USER_2_ACCOUNT, USER_3_ACCOUNT, USER_4_ACCOUNT,
    };
    use update_associated_keys::constants::{
        RUNTIME_ARG_ASSOCIATED_KEY, RUNTIME_ARG_NEW_KEY_WEIGHT,
    };
    use update_thresholds::constants::{
        RUNTIME_ARG_NEW_DEPLOYMENT_THRESHOLD, RUNTIME_ARG_NEW_KEY_MANAGEMENT_THRESHOLD,
    };

    #[test]
    fn should_update_primary_key_weight() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

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
        let actual_weight = account.associated_keys().get(&DEFAULT_ACCOUNT_ADDR);
        assert_eq!(actual_weight, Some(&EXPECTED_KEY_WEIGHT));
    }

    #[test]
    fn should_add_new_accounts_to_primary() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_1_ACCOUNT,
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
        let actual_weight = account.associated_keys().get(&USER_1_ACCOUNT);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));

        // Add User Account 2 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_2_ACCOUNT,
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
        let actual_weight = account.associated_keys().get(&USER_2_ACCOUNT);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));

        // Add User Account 3 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_3_ACCOUNT,
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
        let actual_weight = account.associated_keys().get(&USER_3_ACCOUNT);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));

        // Add User Account 4 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_4_ACCOUNT,
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
        let actual_weight = account.associated_keys().get(&USER_4_ACCOUNT);
        assert_eq!(actual_weight, Some(&DEPLOYMENT_WEIGHT));
    }

    #[test]
    fn should_update_primary_key_weight_and_thresholds() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

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
        let actual_weight = account.associated_keys().get(&DEFAULT_ACCOUNT_ADDR);
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
        let key_mgmt_threshold = account.action_thresholds().key_management();
        // dbg!(key_mgmt_threshold);
        let deployment_threshold = account.action_thresholds().deployment();
        assert_eq!(Some(key_mgmt_threshold), Some(&KEY_MGMT_THRESHOLD));
        assert_eq!(Some(deployment_threshold), Some(&DEPLOYMENT_THRESHOLD));
    }

    #[test]
    fn should_add_two_keys_and_remove_one() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        // Add User Account 1 to the Default Account Associated Keys
        let contract_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            ADD_ACCOUNT_WASM,
            runtime_args! {
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_1_ACCOUNT,
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
                RUNTIME_ARG_NEW_ASSOCIATED_KEY => USER_2_ACCOUNT,
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
                RUNTIME_ARG_REMOVE_ASSOCIATED_KEY => USER_1_ACCOUNT,
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

        // TODO missing_account ?
        let _missing_account = account.associated_keys().get(&USER_1_ACCOUNT);

        let existing_account = account.associated_keys().get(&USER_2_ACCOUNT);

        assert_eq!(existing_account, Some(&DEPLOYMENT_WEIGHT));
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
