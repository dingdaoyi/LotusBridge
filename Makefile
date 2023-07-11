fix:
	cargo fix --allow-dirty
fix-core:
	cd ./protocol/protocol-core && cargo fix --allow-dirty
fix-modbus:
	cd ./protocol/protocol-modbus-tcp && cargo fix --allow-dirty
build-release:
	cargo build --release
run:
	cargo run --release > app.log 2> error.log &
