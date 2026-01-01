BIN := mkinvoice

$(BIN): Cargo.toml Cargo.lock src/*.rs
	cargo build --release
	cp target/release/$(BIN) $(BIN)

install: $(BIN)
	cp $(BIN) ~/.eudoxia.d/bin/$(BIN)

clean:
	cargo clean
	rm -f $(BIN)
