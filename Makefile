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

.PHONY: grpc
grpc:
	@for f in $(SERVICES); do make -C $$f grpc; done

.PHONY: kind
kind:
	kind get clusters -q | grep "^peta" || kind create cluster --config kind.yaml

.PHONY: clean
clean:
	kind delete cluster --name peta
