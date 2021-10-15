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
test:
	@cargo test --workspace

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@grpcurl -plaintext -proto ../../api/peta/fusen/v1/fusen.proto -d '{"title": "テスト", "note": "あああ\nいいいい"}' localhost:50051 peta.fusen.v1.FusenService/Create
