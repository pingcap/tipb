# tipb

TiDB protobuf files

## Requirements

### 1. Install [google/protobuf](https://github.com/google/protobuf)

We use `protoc` 3.5.1, to download: [protobuf/releases/tag/v3.5.1](https://github.com/google/protobuf/releases/tag/v3.5.1)

### 2. Install [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf)

We use `protoc-gen-rust` 1.4.1, to install:

```sh
cargo install --version 1.4.1 protobuf
export PATH="$HOME/.cargo/bin:$PATH"
```

### 3. Install [gogo/protobuf](https://github.com/gogo/protobuf)

We use `protoc-gen-gofast` v0.5, to install:

```sh
go get -u github.com/gogo/protobuf/protoc-gen-gofast
cd $GOPATH/src/github.com/gogo/protobuf
git checkout v0.5
rm $GOPATH/bin/protoc-gen-gofast
go get github.com/gogo/protobuf/protoc-gen-gofast
```

## Generate the Go and Rust codes

```sh
make
```

NOTE: Do not forget to update the dependent projects!
