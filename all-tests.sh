#!/bin/bash

set -e

# move to project root
cd $(dirname `realpath $0`)

cargo test

# assure that benchmark code compiles
pushd bench
cargo build --benches
popd

# build python bindings and run tests
pushd pylovm2
cargo build
pytest
popd

# test the shared object extension
pushd lovm2_extend/examples/primitives
cargo build
cargo test
popd

pushd lovm2_extend/examples/so_module
cargo build
popd

pushd lovm2_extend/examples/custom-objects
cargo build
cargo test
popd

pushd lovm2_std
for submodule in $(ls | grep lovm2); do
    pushd $submodule
    cargo build
    cargo test
    popd
done

cargo build
cargo test
popd
