fix:
	cargo fix --allow-dirty
fix-core:
	cd ./protocol/protocol-core && cargo fix --allow-dirty
fix-modbus:
	cd ./protocol/protocol-modbus-tcp && cargo fix --allow-dirty
build-release:
	cargo build --release
macos-build-arm:
	cargo build --target=armv7-unknown-linux-musleabihf --release
macos-build-aarch64:
	cargo build --release --target=aarch64-unknown-linux-gnu
run:
	cargo run --release > app.log 2> error.log &
