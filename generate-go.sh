#!/bin/bash

set -ex

cd proto
echo "generate go code..."
go install github.com/gogo/protobuf/protoc-gen-gofast
protoc -I.:${GOGO_PROTOBUF} \
    --gofast_out=plugins=grpc:../go-tipb *.proto

cd ../go-tipb
sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
rm -f *.bak
go run golang.org/x/tools/cmd/goimports -w *.pb.go
