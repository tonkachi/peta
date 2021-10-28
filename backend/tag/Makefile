SHELL = /bin/bash

TAG_DATABASE_URL ?= postgres://postgres:postgres@localhost/tag
TAG_TEST_DATABASE_URL ?= postgres://postgres:postgres@localhost/tag_test
MAKEFILE_PATH = $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR_PATH = $(patsubst %/,%,$(dir $(MAKEFILE_PATH)))
INFRA_POSTGRES_DIR_PATH ?= ${MAKEFILE_DIR_PATH}/infrastructure/src/repository/postgres
DIESEL_FILE_PATH ?= ${INFRA_POSTGRES_DIR_PATH}/diesel.toml
DIESEL_MIGRATION_DIR_PATH ?= ${INFRA_POSTGRES_DIR_PATH}/migrations

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
	@cd infrastructure; diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_DATABASE_URL} setup --migration-dir ${DIESEL_MIGRATION_DIR_PATH}
	@cd infrastructure; diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_DATABASE_URL} migration --migration-dir ${DIESEL_MIGRATION_DIR_PATH} run
	@cd infrastructure; diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_DATABASE_URL} migration --migration-dir ${DIESEL_MIGRATION_DIR_PATH} list

.PHONY: db_test
db_test:
	@cd infrastructure; diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_TEST_DATABASE_URL} setup --migration-dir ${DIESEL_MIGRATION_DIR_PATH}
	@cd infrastructure; diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_TEST_DATABASE_URL} migration --migration-dir ${DIESEL_MIGRATION_DIR_PATH} run
	@cd infrastructure; diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_TEST_DATABASE_URL} migration --migration-dir ${DIESEL_MIGRATION_DIR_PATH} list

.PHONY: run
run:
	@cargo run

.PHONY: grpc
grpc:
	@:
