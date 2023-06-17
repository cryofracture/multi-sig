prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cd contracts/add_account && cargo build --release --target wasm32-unknown-unknown
	cd contracts/remove_account && cargo build --release --target wasm32-unknown-unknown
	cd contracts/update_keys && cargo build --release --target wasm32-unknown-unknown
	cd contracts/update_thresholds && cargo build --release --target wasm32-unknown-unknown
	cd contracts/hello_world && cargo build --release --target wasm32-unknown-unknown

	wasm-strip contracts/add_account/target/wasm32-unknown-unknown/release/add_account.wasm | true
	wasm-strip contracts/remove_account/target/wasm32-unknown-unknown/release/remove_account.wasm | true
	wasm-strip contracts/update_keys/target/wasm32-unknown-unknown/release/update_keys.wasm | true
	wasm-strip contracts/update_thresholds/target/wasm32-unknown-unknown/release/update_thresholds.wasm | true
	wasm-strip contracts/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm | true

test: build-contracts
	mkdir -p tests/wasm
	cp contracts/add_account/target/wasm32-unknown-unknown/release/add_account.wasm tests/wasm
	cp contracts/remove_account/target/wasm32-unknown-unknown/release/remove_account.wasm tests/wasm
	cp contracts/update_keys/target/wasm32-unknown-unknown/release/update_associated_keys.wasm tests/wasm
	cp contracts/update_thresholds/target/wasm32-unknown-unknown/release/update_thresholds.wasm tests/wasm
	cp contracts/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings
	cd tests && cargo clippy --release --target wasm32-unknown-unknown -- -D warnings

check-lint: clippy
	cd contracts && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contracts && cargo fmt
	cd tests && cargo fmt

clean:
	cd contracts/add_account/ && cargo clean
	cd contracts/remove_account/ && cargo clean
	cd contracts/update_keys/ && cargo clean
	cd contracts/update_thresholds/ && cargo clean
	cd contracts/hello_world/ && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
