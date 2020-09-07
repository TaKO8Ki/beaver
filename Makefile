test:
	cargo test --no-fail-fast -- --nocapture

doc:
	cargo clean --doc
	cargo doc --no-deps
