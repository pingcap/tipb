#!/usr/bin/env bash

set -ex

cd proto
echo "generate go code..."
go install github.com/gogo/protobuf/protoc-gen-gofast

# Generate the main proto files
protoc -I.:../include --gofast_out=plugins=grpc:../go-tipb *.proto

# Generate tici proto file with a different package name
protoc -I.:../include --gofast_out=plugins=grpc,Mtici/indexer.proto=github.com/pingcap/tipb/go-tipb,import_path=tipb:../go-tipb tici/*.proto

cd ../go-tipb
if [ -f "tici/indexer.pb.go" ]; then
  mv tici/indexer.pb.go tici.pb.go
fi
sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
sed -i.bak -E 's/context \"context\"//g' *.pb.go
sed -i.bak -E 's/fmt \"fmt\"//g' *.pb.go
sed -i.bak -E 's/io \"io\"//g' *.pb.go
sed -i.bak -E 's/math \"math\"//g' *.pb.go
sed -i.bak -E 's/_ \".*rustproto\"//g' *.pb.go
sed -i.bak -E '/^[[:space:]]*_ \"\.\"[[:space:]]*$/d' *.pb.go
sed -i.bak -E 's/package tipb_tici/package tipb/' *.pb.go

# Remove temporary files
rm -f *.bak
if [ -d "tici" ]; then
  rm -rf tici
fi

go run golang.org/x/tools/cmd/goimports -w *.pb.go
