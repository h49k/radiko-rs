release:
	cargo build --release

install:
	install target/release/radiko-rs /usr/local/bin
