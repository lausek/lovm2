#!/bin/bash

set -e

# move to project root
cd $(dirname `realpath $0`)

cargo test

# assure that benchmark code compiles
pushd bench
env RUSTFLAGS="--cfg lovm2_version=\"0.4.8\"" cargo build --benches
popd

# build python bindings and run tests
pushd pylovm2
cargo build
pytest
popd

# test the shared object extension
pushd src/lovm2_extend
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
pushd src/lovm2_std
cargo build
cargo test
popd
