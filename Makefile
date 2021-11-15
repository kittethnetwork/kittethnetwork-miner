# Make Script For Miner Rust
BIN_DIR	  	:= bin/

release: $(BIN_DIR)main.bin

# Build The Rust Project, .cargo and Cargo.Toml hold the flags for this
$(BIN_DIR)main.bin:
	cargo build --release

# Clean The Build Folder To Allow For A Complete Rebuild
clean:
	rm -f $(BIN_DIR)*.o
	rm -f $(BIN_DIR)*.bin
	cargo clean

run:
	cargo run --release

test:
	cargo run

setup:
	mkdir bin

