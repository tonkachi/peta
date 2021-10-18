SHELL = /bin/bash

.PHONY: all
all: fmt lint test

.PHONY: fmt
fmt:
	@cargo fmt --all

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: test
test: db_test
	@cargo test --workspace

.PHONY: db
db:
	@:

.PHONY: db_test
db_test:
	@:

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@:
