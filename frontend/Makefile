# vim:ft=make

.PHONY: hotreload
hotreload:
	watchexec -r -s SIGTERM -- $(MAKE) dev

.PHONY: dev
dev:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --target no-modules --out-dir target --out-name koumakan $$HOME/.cargo/target/wasm32-unknown-unknown/debug/koumakan_frontend.wasm --no-typescript
	cp index.html ./target/
	cd target && python -m http.server 8080

.PHONY: release
release:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen --target no-modules --out-dir target --out-name koumakan $$HOME/.cargo/target/wasm32-unknown-unknown/release/koumakan_frontend.wasm --no-typescript
	cp index.html ./target/
	cd target && python -m http.server 8080

clean:
	rm -rf target
