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
pushd lovm2_extend
for example in $(ls ./examples); do
    pushd ./examples/$example
    cargo build
    popd
done

for example in $(ls ./examples); do
    pushd ./examples/$example
    cargo test
    popd
done

cargo build
popd

# test standard library
pushd lovm2_std
for submodule in $(ls | grep lovm2); do
    pushd $submodule
    cargo build
    popd
done

for submodule in $(ls | grep lovm2); do
    pushd $submodule
    cargo test
    popd
done

cargo build
cargo test
popd
