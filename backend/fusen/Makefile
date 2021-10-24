SHELL = /bin/bash

FUSEN_DATABASE_URL ?= postgres://postgres:postgres@localhost/fusen
FUSEN_TEST_DATABASE_URL ?= postgres://postgres:postgres@localhost/fusen_test

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
	@diesel --database-url ${FUSEN_DATABASE_URL} setup
	@diesel --database-url ${FUSEN_DATABASE_URL} migration run
	@diesel --database-url ${FUSEN_DATABASE_URL} migration list

.PHONY: db_test
db_test:
	@diesel --database-url ${FUSEN_TEST_DATABASE_URL} setup
	@diesel --database-url ${FUSEN_TEST_DATABASE_URL} migration run
	@diesel --database-url ${FUSEN_TEST_DATABASE_URL} migration list

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@grpcurl -plaintext -proto ../../api/peta/fusen/v1/fusen.proto -d '{"title": "テスト", "note": "あああ\nいいいい"}' localhost:50051 peta.fusen.v1.FusenService/Create
	@grpcurl -plaintext -proto ../../api/peta/fusen/v1/fusen.proto -d '{"id": "01FJHPYFY4K5QDKVBPYJ2GJEGQ"}' localhost:50051 peta.fusen.v1.FusenService/Get
	@grpcurl -plaintext -proto ../../api/peta/fusen/v1/fusen.proto -d '{"id": "01FJHPYFY4K5QDKVBPYJ2GJEGQ"}' localhost:50051 peta.fusen.v1.FusenService/Delete
