# tipb

TiDB protobuf files

## Requirements

### Install [google/protobuf](https://github.com/google/protobuf)

We use `protoc` 3.5.1, to download: [protobuf/releases/tag/v3.5.1](https://github.com/google/protobuf/releases/tag/v3.5.1)

## Proto formatting

To avoid IDE-induced diffs, we use `buf format` to keep `.proto` files consistently formatted.

- Check formatting (also runs as part of `make check`): `make proto-fmt-check`
- Format in-place: `make proto-fmt`

## Generate the Go and Rust codes

```sh
make
```

NOTE: Do not forget to update the dependent projects!
