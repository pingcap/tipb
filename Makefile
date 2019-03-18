all: go rust binlog

go:
	./generate-go.sh

rust:
	cargo build --features gen

binlog:
	./generate-binlog.sh
