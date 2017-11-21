# tipb
TiDB protobuf files

# Usage

### install protobuf compiler.

+ Can be downloaded at https://github.com/google/protobuf

### Write your own protocol file in proto folder.

### Make sure the gogo protobuf is installed and checked out to v0.5
+ `go get -u github.com/gogo/protobuf/proto`
+ `cd $GOPATH/src/github.com/gogo/protobuf`
+ `git checkout v0.5`

### Generate go and rust code.
We generate all go code in pkg folder and rust in src folder.

+ `make go`
+ `make rust`

### Update the dependent projects.
