cd proto

echo "generate go code..."
protoc --go_out=../go-tipb *.proto
