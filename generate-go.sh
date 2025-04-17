#!/bin/bash

set -ex

cd proto
echo "generate go code..."
go install github.com/gogo/protobuf/protoc-gen-gofast
protoc -I.:${GOGO_PROTOBUF} \
    --gofast_out=plugins=grpc:../go-tipb *.proto

# Generate TiCI proto with the main tipb Go package.
protoc -I.:${GOGO_PROTOBUF} \
    --gofast_out=plugins=grpc,Mtici/indexer.proto=github.com/pingcap/tipb/go-tipb,import_path=tipb:../go-tipb tici/*.proto

cd ../go-tipb
mv tici/indexer.pb.go tici.pb.go
sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
sed -i.bak -E 's/import _ \"\.\"//g' *.pb.go
sed -i.bak -E 's/package tipb_tici/package tipb/' *.pb.go
rm -f *.bak
go run golang.org/x/tools/cmd/goimports -w *.pb.go
