#!/bin/bash

set -e

# move to project root
cd $(dirname `realpath $0`)

cargo test

# assure that benchmark code compiles
pushd crates/lovm2_bench
env RUSTFLAGS="--cfg lovm2_version=\"0.4.8\"" cargo build --benches
popd

# build python bindings and run tests
pushd crates/lovm2_python
cargo build
pytest
popd

pushd crates/lovm2_core
cargo test
popd

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
cargo doc

# test standard library
pushd crates/lovm2_std
cargo build
cargo test
popd

