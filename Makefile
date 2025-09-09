.PHONY: all go rust c++

CURDIR := $(shell pwd)

export GOBIN=$(CURDIR)/bin
export PATH := $(CURDIR)/bin/:$(PATH)

all: go rust c++

dependence:
	go mod download

check: dependence
	./scripts/check.sh

go: dependence check
	./scripts/generate-go.sh
	GO111MODULE=on go mod tidy
	GO111MODULE=on go build ./go-tipb/...

rust: check
	cargo check && \
	cargo check --no-default-features --features prost-codec

c++: dependence
	./scripts/generate-cpp.sh

tipb.a:
	mkdir -p cpp/build && cd cpp/build && cmake -DCMAKE_BUILD_TYPE=Release .. && make tipb
