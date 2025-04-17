#!/usr/bin/env bash

check_protoc_version() {
    version=$(protoc --version | awk '{print $NF}')
    major=$(echo ${version} | cut -d '.' -f 1)
    minor=$(echo ${version} | cut -d '.' -f 2)
    if [ "$major" -eq 3 ] && [ "$minor" -ge 5 ]; then
        return 0
    fi
    # protobuf bumps the major version to 21 after 3.
    # https://github.com/protocolbuffers/protobuf/releases/tag/v21.7
    if [ "$major" -ge 21 ]; then
        return 0
    fi
    echo "protoc version not match, version 3.5.x+ is needed, current version: ${version}"
    return 1
}

check-protos-compatible() {
    GOPATH=$(go env GOPATH)
    if [ -z $GOPATH ]; then
        printf "Error: the environment variable GOPATH is not set, please set it before running %s\n" $PROGRAM > /dev/stderr
        exit 1
    fi
    export PATH=$GOPATH/bin:$PATH

    if [ ! -f "$GOPATH/bin/protolock" ]; then
        GO111MODULE=off go get github.com/nilslice/protolock/cmd/protolock
        GO111MODULE=off go install github.com/nilslice/protolock/cmd/protolock
    fi

    if protolock status -lockdir=scripts -protoroot=proto; then
        protolock commit -lockdir=scripts -protoroot=proto
    else
        echo "Meet break compatibility problem, please check the code."
        # In order not to block local branch development, when meet break compatibility will force to update `proto.lock`.
        protolock commit --force -lockdir=scripts -protoroot=proto
    fi
    # git report error like "fatal: detected dubious ownership in repository at" when reading the host's git folder
    git config --global --add safe.directory "$(pwd)"
    # If the output message is encountered, please add proto.lock to git as well.
    git diff scripts/proto.lock | cat
    git diff --quiet scripts/proto.lock
    if [ $? -ne 0 ]; then
        echo "Please add proto.lock to git."
        return 1
    fi
    return 0
}

check-protos-options() {
    local options=(
        'import "gogoproto/gogo.proto";'
        'import "rustproto.proto";'

        'option (gogoproto.sizer_all) = true;'
        'option (gogoproto.marshaler_all) = true;'
        'option (gogoproto.unmarshaler_all) = true;'
        # Remove unnecessary fields from pb structs.
        # XXX_NoUnkeyedLiteral struct{} `json:"-"`
        # XXX_unrecognized     []byte   `json:"-"`
        # XXX_sizecache        int32    `json:"-"`
        'option (gogoproto.goproto_unkeyed_all) = false;'
        'option (gogoproto.goproto_unrecognized_all) = false;'
        'option (gogoproto.goproto_sizecache_all) = false;'
        # TiKV does not need reflection and descriptor data.
        'option (rustproto.lite_runtime_all) = true;'
    )

    local folder="./proto"
    for pb in "$folder"/*; do
        # Skip directories
        if [ -d "$pb" ]; then
            continue
        fi
        
        # Iterate through the array
        for option in "${options[@]}"; do
            if ! grep -q "$option" "$pb"; then
                echo "Please add option \"$option\" to $pb"
                return 1
            fi
        done
    done

    return 0
}

if ! check_protoc_version || ! check-protos-compatible || ! check-protos-options; then
    exit 1
fi
