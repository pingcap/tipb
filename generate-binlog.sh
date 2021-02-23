#!/usr/bin/env bash

set -ex

cd proto/binlog

echo "generate binlog code..."
protoc -I.:${GOGO_PROTOBUF} --gofast_out=../../go-binlog binlog.proto
protoc -I.:${GOGO_PROTOBUF} --gofast_out=plugins=grpc:../../go-binlog pump.proto
protoc -I.:${GOGO_PROTOBUF} --gofast_out=plugins=grpc:../../go-binlog cistern.proto
cd ../../go-binlog
sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
rm -f *.bak
go run golang.org/x/tools/cmd/goimports -w *.pb.go
