#!/bin/bash

set -e

# move to project root
cd $(dirname `realpath $0`)

cargo install --path .

LOVM2_USER_DIR=`realpath ~/.local/lib/lovm2`

pushd ./lovm2_std/

#echo "building std ..."
#cargo build --release --features=net

echo "installing std ..."
mkdir -p $LOVM2_USER_DIR

function build_and_install()
{
    pushd $1

    cargo build --release
    sh_name=$(find ./target/release -maxdepth 1 -name "liblovm2*.so")
    new_name=$(echo $sh_name | awk -F_ '{print $NF}')
    cp $sh_name $LOVM2_USER_DIR/$new_name

    popd
}

build_and_install lovm2_std_buffer
build_and_install lovm2_std_collection
build_and_install lovm2_std_fs
build_and_install lovm2_std_functional
build_and_install lovm2_std_json
build_and_install lovm2_std_math
build_and_install lovm2_std_net
build_and_install lovm2_std_regex
build_and_install lovm2_std_string
