#!/bin/bash
set -euo pipefail

function sed_inplace()
{
    if sed --help 2>/dev/null | grep GNU > /dev/null; then
        sed -i "$@"
    else
        sed -i '' "$@"
    fi
}

# Clean up special marks for gogoproto
function clean_up()
{
    local file=$1
    sed_inplace '/gogo.proto/d' ${file}
    sed_inplace '/option\ (gogoproto/d' ${file}
    sed_inplace -e 's/\[.*gogoproto.*\]//g' ${file}
}

if [ "$#" -eq 2 ]; then
    cmd="$1"
    file="$2"
    # <prog> clean ${file} -- clean up special marks
    if [[ ${cmd} != "clean" || ! -f ${file} ]]; then
        echo "Fail to clean up original file. cmd=${cmd}, file=${file}" >&2
        exit 1
    fi
    clean_up ${file}
    exit 0
fi

# Clean up special marks and re-generate .pb.h, .pb.cc files
rm -rf proto-cpp && mkdir -p proto-cpp
rm -rf cpp/tipb && mkdir cpp/tipb

cp proto/*.proto proto-cpp/

for file in `ls proto-cpp/*`; do
    clean_up ${file}
done

cd proto-cpp
echo "generate cpp code..."
protoc --cpp_out=../cpp/tipb/ *.proto
cd ..

rm -rf proto-cpp
