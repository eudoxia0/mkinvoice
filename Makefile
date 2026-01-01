BIN := mkinvoice

$(BIN): Cargo.toml Cargo.lock src/*.rs
	cargo build --release
	cp target/release/$(BIN) $(BIN)

clean:
	cargo clean
	rm -f $(BIN)
