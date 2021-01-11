#!/bin/bash

set -e

echo "release script. use --skip-python to avoid publishing pylovm2"
echo ""

# move to project root
cd $(dirname `realpath $0`)

cargo publish

if [[ "$@" != *"--skip-python"* ]]; then
    pushd ./pylovm2
    docker build -t pylovm2-build .
    docker run -it -v $(pwd):/io pylovm2-build maturin publish
    popd
fi

echo "new version released. congrats!"
