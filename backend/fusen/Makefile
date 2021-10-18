SHELL = /bin/bash

DATABASE_URL ?= postgres://postgres:postgres@localhost/peta

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

.PHONY: db
db:
	@diesel --database-url ${DATABASE_URL} setup
	@diesel --database-url ${DATABASE_URL} migration run
	@diesel --database-url ${DATABASE_URL} migration list

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@grpcurl -plaintext -proto ../../api/peta/fusen/v1/fusen.proto -d '{"title": "テスト", "note": "あああ\nいいいい"}' localhost:50051 peta.fusen.v1.FusenService/Create
