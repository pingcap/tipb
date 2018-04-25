# tipb

TiDB protobuf files

## PreRequests

1. (google/protobuf)[https://github.com/google/protobuf], v3.5.1
2. (gogo/protobuf)[https://github.com/gogo/protobuf], v0.5
3. (stepancheg/rust-protobuf)[https://github.com/stepancheg/rust-protobuf]

```sh
# install protoc
git clone https://github.com/google/protobuf
cd protobuf
git checkout v3.5.1
./autogen.sh
mkdir install
./configure --prefix=${PWD}/install
make -j8
make install
export PTAH=${PWD}/install/bin:$PATH

# install protoc-gen-rust
cargo install protobuf
export PATH="$HOME/.cargo/bin:$PATH"

# install protoc-gen-gofast
# Make sure the gogo protobuf and protoc-gen-gofast is installed and checked out to v0.5
go get -u github.com/gogo/protobuf/protoc-gen-gofast
cd $GOPATH/src/github.com/gogo/protobuf
git checkout v0.5
rm $GOPATH/bin/protoc-gen-gofast
go get github.com/gogo/protobuf/protoc-gen-gofast
```

## Generate go and rust code

```sh
make
```

NOTE: Do not forget to update the dependent projects !
