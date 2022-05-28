
rust.bin: src/main.rs
	cargo build --release
	riscv64-unknown-elf-objcopy -O binary target/riscv32i-unknown-none-elf/release/example rust.bin
	riscv64-unknown-elf-objdump -d target/riscv32i-unknown-none-elf/release/example
