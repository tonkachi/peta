SHELL = /bin/bash

TAG_DATABASE_URL ?= postgres://postgres:postgres@localhost/tag
TAG_TEST_DATABASE_URL ?= postgres://postgres:postgres@localhost/tag_test
MAKEFILE_PATH = $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR_PATH = $(patsubst %/,%,$(dir $(MAKEFILE_PATH)))
DIESEL_FILE_PATH ?= ${MAKEFILE_DIR_PATH}/diesel.toml

.PHONY: all
all: db

.PHONY: db
db:
	@diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_DATABASE_URL} setup
	@diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_DATABASE_URL} migration run
	@diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_DATABASE_URL} migration list

.PHONY: db_test
db_test:
	@diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_TEST_DATABASE_URL} setup
	@diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_TEST_DATABASE_URL} migration run
	@diesel --config-file ${DIESEL_FILE_PATH} --database-url ${TAG_TEST_DATABASE_URL} migration list
