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
