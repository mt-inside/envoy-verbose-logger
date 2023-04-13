default:
	@just --list

install-tools:
	rustup toolchain install nightly
	rustup target add wasm32-unknown-unknown
	# Need fix for https://github.com/rustwasm/wasm-pack/issues/952
	cargo install --git https://github.com/comtrya/comtrya --branch main -- comtrya

build-release: clean
	wasm-pack build --release

build-debug: clean
	cargo build --target=wasm32-unknown-unknown

clean:
	cargo clean
	rm -rf ./pkg
