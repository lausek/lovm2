#!/bin/bash

set -e

echo "RELEASE: use arguments --skip-error and --skip-internals to prevent publishing subcrates"
echo ""

# move to project root
cd $(dirname `realpath $0`)

if [[ "$@" != *"--skip-error"* ]]; then
    pushd ./src/lovm2_error
    cargo publish
    popd
fi

if [[ "$@" != *"--skip-internals"* ]]; then
    pushd ./src/lovm2_internals
    cargo publish
    popd
fi

cargo publish

if [[ "$@" != *"--skip-python"* ]]; then
    pushd ./pylovm2
    docker build -t pylovm2-build .
    docker run -it -v $(pwd):/io pylovm2-build maturin publish
    popd
fi

echo "new version released. congrats!"
