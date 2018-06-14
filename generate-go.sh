cd proto

echo "generate go code..."
GOGO_ROOT=${GOPATH}/src/github.com/gogo/protobuf
if [ ! -d $GOGO_ROOT ]; then
	echo "please use the following command to get specific version of gogo.\n\n"
	echo "go get -u github.com/gogo/protobuf/protoc-gen-gofast"
	echo "cd ${GOPATH}/src/github.com/gogo/protobuf"
	echo "git checkout v0.5"
	echo "rm ${GOPATH}/bin/protoc-gen-gofast"
	echo "go get github.com/gogo/protobuf/protoc-gen-gofast"
else
	protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf --gofast_out=../go-tipb *.proto
	cd ../go-tipb
	sed -i.bak -E 's/import _ \"gogoproto\"//g' *.pb.go
	sed -i.bak -E 's/import fmt \"fmt\"//g' *.pb.go
	rm -f *.bak
	goimports -w *.pb.go
fi

