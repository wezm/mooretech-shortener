default: build/mooretech_shortener.js

target/wasm32-unknown-unknown/release/mooretech_shortener.wasm: src/*.rs Cargo.toml
	cargo build --release --target wasm32-unknown-unknown

build/mooretech_shortener.js: target/wasm32-unknown-unknown/release/mooretech_shortener.wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/mooretech_shortener.wasm --target deno --out-dir build

run: build/mooretech_shortener.js
	deno run --allow-read --allow-net --allow-env src/index.ts

test: build/mooretech_shortener.js
	deno test --allow-read --allow-net tests/integration.ts

.PHONY: run test
