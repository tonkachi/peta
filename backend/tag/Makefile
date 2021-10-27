SHELL = /bin/bash

MAKEFILE_PATH = $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR_PATH = $(patsubst %/,%,$(dir $(MAKEFILE_PATH)))
INFRA_WORKSPACE_PATH ?= ${MAKEFILE_DIR_PATH}/infrastructure

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
	@make -C ${INFRA_WORKSPACE_PATH} db

.PHONY: db_test
db_test:
	@make -C ${INFRA_WORKSPACE_PATH} db_test

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@:
