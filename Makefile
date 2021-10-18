SHELL = /bin/bash

SERVICES = \
	backend/fusen \
	backend/tag \
	backend/recommendation \

.PHONY: all
all: fmt lint test

.PHONY: fmt
fmt:
	@for f in $(SERVICES); do make -C $$f fmt; done

.PHONY: lint
lint:
	@for f in $(SERVICES); do make -C $$f lint; done

.PHONY: test
test:
	@for f in $(SERVICES); do make -C $$f test; done

.PHONY: db
db:
	@cargo install diesel_cli --no-default-features --features postgres
	@for f in $(SERVICES); do make -C $$f db; done

.PHONY: db_test
db_test:
	@cargo install diesel_cli --no-default-features --features postgres
	@for f in $(SERVICES); do make -C $$f db_test; done

.PHONY: grpc
grpc:
	@for f in $(SERVICES); do make -C $$f grpc; done
