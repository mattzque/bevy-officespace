# some build commands for local development

.PHONY: wasm

build:
	cargo build --release --no-default-features

wasm:
	cargo build --release --no-default-features --target wasm32-unknown-unknown
	mkdir -p wasm/assets || true
	cp assets/* wasm/assets || true
	cd wasm/ && python3 -m http.server
