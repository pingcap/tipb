cd proto

echo "generate rust code..."
ret=0
protoc --rust_out ../src *.proto || ret=$?

echo "extern crate protobuf;" > ../src/lib.rs
for file in `ls *.proto`
    do
    base_name=$(basename $file ".proto")
    echo "#[cfg_attr(rustfmt, rustfmt_skip)]" >> ../src/lib.rs
    echo "pub mod $base_name;" >> ../src/lib.rs
done
if [[ $ret -ne 0 ]]; then
    exit $ret
fi
cd ..
cargo build
