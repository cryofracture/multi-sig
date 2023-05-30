prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cd contracts/add_account && cargo build --release --target wasm32-unknown-unknown
	cd contracts/remove_account && cargo build --release --target wasm32-unknown-unknown
	cd contracts/update_keys && cargo build --release --target wasm32-unknown-unknown
	cd contracts/update_thresholds && cargo build --release --target wasm32-unknown-unknown

	wasm-strip contracts/add_account/target/wasm32-unknown-unknown/release/add_account.wasm 2>/dev/null | true
	wasm-strip contracts/remove_account/target/wasm32-unknown-unknown/release/remove_account.wasm 2>/dev/null | true
	wasm-strip contracts/update_keys/target/wasm32-unknown-unknown/release/update_keys.wasm 2>/dev/null | true
	wasm-strip contracts/update_thresholds/target/wasm32-unknown-unknown/release/update_thresholds.wasm 2>/dev/null | true

test: build-contracts
	mkdir -p tests/wasm
	cp contracts/add_account/target/wasm32-unknown-unknown/release/add_account.wasm tests/wasm
	cp contracts/remove_account/target/wasm32-unknown-unknown/release/remove_account.wasm tests/wasm
	cp contracts/update_keys/target/wasm32-unknown-unknown/release/update_keys.wasm tests/wasm
	cp ontracts/update_thresholds/target/wasm32-unknown-unknown/release/update_thresholds.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contracts/add_account/ && cargo clean
	cd contracts/remove_account/ && cargo clean
	cd contracts/update_keys/ && cargo clean
	cd ontracts/update_thresholds/ && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
