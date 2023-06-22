# Multi-signature Key Management on the Casper Network

# Step 1: Clone and build multi-sig contracts

```
git clone https://github.com/cryofracture/multi-sig && cd multi-sig
```

# Step 2: Prepare Contracts
```
rustup update
make clean
make prepare
make build-contracts
```

# Step 3: Update primary key weight to set thresholds
The included contracts will set account weights to 1, requiring all accounts to multi-sign for deployments or key management controls. Initially, we have to set the weight of the primary key high enough to increase the key_management threshold.

First, retrieve the account-hash of the key you are working with, via cspr.live or `casper-client account-address –public-key XXX`

`casper-client account-address --public-key <INSERT_PUBLIC_KEY_HEX>`

Once the account-hash is obtained, you can update the weight of the primary key by calling the update_keys contract:

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING
```
casper-client put-deploy --node-address https://rpc.testnet.casperlabs.io/ \
--chain-name "casper-test" \
--payment-amount 2000000000 \
--secret-key /path/to/keys_dir/secret_key.pem \
--session-path contracts/update_keys/target/wasm32-unknown-unknown/release/update_associated_keys.wasm \
--session-arg "associated_key:account_hash='account-hash-<ACCOUNT_HASH_HEX_HERE>'" \
--session-arg "new_weight:u8='3'"
```
Which results in the main key now showing weight “3”:
```
"Account": {
      "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
      "action_thresholds": {
        "deployment": 1,
        "key_management": 1
      },
      "associated_keys": [
        {
          "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
          "weight": 3
        }
      ],
      "main_purse": "uref-b4532f30031b9deb8b2879a91ac185577dcba763de9d48753385e0ef41235dfa-007",
      "named_keys": []
    }
  }
```


# Step 4: Update Thresholds
After the primary key has had its weight increased, you can set the deployment and key_management thresholds for the account to set up a multi-sign environment. In order to add a new key to the account, call the add_account.wasm contract. The included contracts will set deployment threshold to 2(hard coded into the contract), and key_management to 3. It is recommended to have more weight on the account than is needed, by a small amount, to ensure you can meet key_management needs as necessary.

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING
```
casper-client put-deploy \
--node-address https://rpc.testnet.casperlabs.io \
--chain-name casper-test \
--payment-amount 5000000000 \
--secret-key /path/to/keys_dir/secret_key.pem \
--session-path contracts/update_thresholds/target/wasm32-unknown-unknown/release/update_thresholds.wasm
```
#Step 5: Add a new key to the account

To add `account-hash-e2d00525cac31ae2756fb155f289d276c6945b6914923fe275de0cb127bffee7`, for example, to the account to set up multi-signature, you would issue a deploy command like:


### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING
```
casper-client put-deploy --node-address https://rpc.testnet.casperlabs.io/ \
--chain-name "casper-test" \
--payment-amount 50000000000 \
--secret-key /path/to/keys_dir/secret_key.pem \
--session-path contracts/add_account/target/wasm32-unknown-unknown/release/add_account.wasm \
--session-arg "new_key:account_hash='account-hash-e2d00525cac31ae2756fb155f289d276c6945b6914923fe275de0cb127bffee7" \
--session-arg "weight:u8='1'"
```
NOTES:
The key_management threshold cannot be changed before the deployment threshold can be changed,
The primary key on the account can lower its weight by itself if it has enough weight to meet key_mangement.
The above configuration assumes 3 keys to an account to manage keys, but only 2 to deploy against the account. The third pair and beyond should be kept incredibly secure to ensure robustness of the integrity of the account.

This will result in the account’s action_thresholds now looking like:
```
"action_thresholds": {
        "deployment": 2,
        "key_management": 3
      },
```


# Step 6: Add and remove a “compromised” dummy key to the Account

Assuming the below account structure:
```
"Account": {
      "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
      "action_thresholds": {
        "deployment": 2,
        "key_management": 3
      },
      "associated_keys": [
        {
          "account_hash": "account-hash-04a9691a9f8f05a0f08bd686f188b27c7dbcd644b415759fd3ca043d916ea02f",
          "weight": 1
        },
        {
          "account_hash": "account-hash-1fed34baa6807a7868bb18f91b161d99ebf21763810fe4c92e39775d10bbf1f8",
          "weight": 1
        },
        {
          "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
          "weight": 3
        },
        {
          "account_hash": "account-hash-e2d00525cac31ae2756fb155f289d276c6945b6914923fe275de0cb127bffee7",
          "weight": 1
        }
      ],
      "main_purse": "uref-b4532f30031b9deb8b2879a91ac185577dcba763de9d48753385e0ef41235dfa-007",
      "named_keys": []
    }
```


In this scenario, `account-hash-04a9691a9f8f05a0f08bd686f188b27c7dbcd644b415759fd3ca043d916ea02f` has become compromised and should no longer have write access to the account. To remove this account, call the remove_account.wasm with a command like:

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING

```
casper-client put-deploy \
--node-address https://rpc.testnet.casperlabs.io \
--chain-name casper-test \
payment-amount 5000000000 \
--secret-key /path/to/keys_dir/secret_key.pem \
 --session-path contracts/remove_account/target/wasm32-unknown-unknown/release/remove_account.wasm \
--session-arg "remove_key:account_hash='account-hash-04a9691a9f8f05a0f08bd686f188b27c7dbcd644b415759fd3ca043d916ea02f'"
```
Which now returns an account without account-hash-04a96…ea02f:
```
"Account": {
      "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
      "action_thresholds": {
        "deployment": 2,
        "key_management": 3
      },
      "associated_keys": [
        {
          "account_hash": "account-hash-1fed34baa6807a7868bb18f91b161d99ebf21763810fe4c92e39775d10bbf1f8",
          "weight": 1
        },
        {
          "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
          "weight": 3
        },
        {
          "account_hash": "account-hash-e2d00525cac31ae2756fb155f289d276c6945b6914923fe275de0cb127bffee7",
          "weight": 1
        }
      ],
      "main_purse": "uref-b4532f30031b9deb8b2879a91ac185577dcba763de9d48753385e0ef41235dfa-007",
      "named_keys": []
    }

```
# Step 7: Lower the primary key weight after set-up

After all associated keys and action_thresholds have been set on the account to the desired points, you can self-deploy a contract call that will lower the weight of the original key back to 1, allowing proper multi-signature setup (again requiring 2 signatures to deploy, 3 weight for key_management). To lower the key’s weight, call the update_associated_keys.wasm contract again:

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING

```
casper-client put-deploy \
--node-address https://rpc.testnet.casperlabs.io \
--chain-name "casper-net-1" \
--payment-amount 5000000000 \
--secret-key /path/to/keys_dir/secret_key.pem \
--session-path contracts/update_keys/target/wasm32-unknown-unknown/release/update_associated_keys.wasm \
--session-arg "associated_key:account_hash='account-hash-<PRIMARY_ACCOUNT_HASH_HEX_HERE>'" \
--session-arg "new_weight:u8='0'"
```
Which will set the account to now look like:
```
"Account": {
      "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
      "action_thresholds": {
        "deployment": 2,
        "key_management": 3
      },
      "associated_keys": [
        {
          "account_hash": "account-hash-1fed34baa6807a7868bb18f91b161d99ebf21763810fe4c92e39775d10bbf1f8",
          "weight": 1
        },
        {
          "account_hash": "account-hash-d89c49f7e03f418dc285e94e254d53574db878665271f366bb3aeddded7ab757",
          "weight": 0
        },
        {
          "account_hash": "account-hash-e2d00525cac31ae2756fb155f289d276c6945b6914923fe275de0cb127bffee7",
          "weight": 1
        }
      ],
      "main_purse": "uref-b4532f30031b9deb8b2879a91ac185577dcba763de9d48753385e0ef41235dfa-007",
      "named_keys": []
    }
```

# Step 8: Set up a multi-sign deploy from the primary account

After the action thresholds are set and keys are on the account in a way that will allow multi-signature deployments, you can easily sign deploys for sending to the network using the “casper-client make-deploy” command. Enough weight against the account must sign the deploy to meet the deployment threshold, in this case 2 signatures. Either party can then send the deployment to the network when ready.

To start signing a deploy, we will start with a simple “hello, world” named key contract.

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING

```
casper-client make-deploy --chain-name casper-test \
--payment-amount 3000000000 \
--session-path contracts/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm \
--secret-key /path/to/keys_dir/secret_key.pem \
--session-arg "my-key-name:string='user_1_key'" \
--session-arg "message:string='Hello, World'" \
--output hello_world_one_signature
```
And then both associated keys can sign the deploy to meet the action threshold for the account:

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING

```
casper-client sign-deploy -i hello_world_one_signature -k ~/cspr_nctl/user-2.pem -o hello_world_ready
```
And you can now send the deploy to the network:

```
casper-client send-deploy --node-address https://rpc.testnet.casperlabs.io -i hello_world_ready
```
Which will deploy to the account and create a named_key:
```
"named_keys": [
        {
          "key": "uref-9b9ecaa9e5e235fc6955d4d528cb1b5b38f2d800f6cbbc55351131a3701b5a81-007",
          "name": "my-key-name"
        }
      ]
```


Step 9: Set-up a multi-sign deploy from an associated key

In order to initiate a deploy signature for a deploy against the primary account, the same steps for signing can be followed, but on the make-deploy command you must add the –session-account argument to tell the execution engine which account to run the contract against. The session-account requires a public key hex.

### FOR EXAMPLE ONLY, PLEASE UPDATE PRIOR TO EXECUTING

```
casper-client make-deploy --chain-name casper-test \
--payment-amount 3000000000 \
--session-path contracts/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm \
--secret-key /path/to/keys_dir/secret_key.pem \
--session-arg "my-key-name:string='user_1_key'" \
--session-arg "message:string='Hello, World'" \
--session-account 017fac40914593d00bc7e6a8f4a0d758d6a12e4e036f1473ae50d124ffd91b103b
--output hello_world_one_signature
```
