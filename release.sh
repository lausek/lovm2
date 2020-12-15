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

echo "new version released. congrats!"
