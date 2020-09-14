.PHONY: test
test:
	@cargo test --no-fail-fast -- --nocapture

.PHONY: dbuild
dbuild:
	@cargo clean --doc
	@cargo doc --no-deps

.PHONY: dopen
dopen:
	@open ./target/doc/beaver/index.html

.PHONY: doc
doc:
	@cargo clean --doc
	@cargo doc --no-deps --open
