# tipb

TiDB protobuf files

## Build with local toolchain

### Install [google/protobuf](https://github.com/google/protobuf)

We use `protoc` 3.5.1, to download: [protobuf/releases/tag/v3.5.1](https://github.com/google/protobuf/releases/tag/v3.5.1)

### Generate the Go and Rust codes

```sh
make
```

## Build with docker

### Build the docker image

```sh
docker build . -t tipb-builder
```

### Generate codes

Basically just use the docker image instead of `make`:

```sh
# Generate All codes
docker run -v $(pwd):/tipb tipb-builder 
# Generate Go codes only
docker run -v $(pwd):/tipb tipb-builder go
# Generate Rust codes only
docker run -v $(pwd):/tipb tipb-builder rust
```

## Note
Do not forget to update the dependent projects!
