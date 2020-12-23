#!/bin/bash

set -e

# move to project root
cd $(dirname `realpath $0`)

cargo install --path .

LOVM2_USER_DIR=`realpath ~/.local/lib/lovm2`

pushd ./lovm2_std/

echo "building std ..."
cargo build --release

echo "installing std ..."
mkdir -p $LOVM2_USER_DIR
for so in $(find ./target/release -maxdepth 1 -name "liblovm2*.so"); do
    new_name=$(echo $so | awk -F_ '{print $NF}')
    cp $so $LOVM2_USER_DIR/$new_name
done
