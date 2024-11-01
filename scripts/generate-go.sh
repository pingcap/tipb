#!/usr/bin/env bash

set -ex

cd proto
echo "generate go code..."
go install github.com/gogo/protobuf/protoc-gen-gofast
protoc -I.:../include --gofast_out=plugins=grpc:../go-tipb *.proto

cd ../go-tipb
sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
sed -i.bak -E 's/context \"context\"//g' *.pb.go
sed -i.bak -E 's/fmt \"fmt\"//g' *.pb.go
sed -i.bak -E 's/io \"io\"//g' *.pb.go
sed -i.bak -E 's/math \"math\"//g' *.pb.go
sed -i.bak -E 's/_ \".*rustproto\"//g' *.pb.go
rm -f *.bak
go run golang.org/x/tools/cmd/goimports -w *.pb.go
