#!/bin/bash

PYLOVM2_DIR=$(dirname `realpath $0`)
cd $PYLOVM2_DIR

pushd $PYLOVM2_DIR/../
LOVM2_DIR=$(pwd)
popd

# Build pylovm2 with `maturin build`
echo "Build pylovm2 now ..."
docker run -ti --entrypoint "/io/build-pylovm2.sh" -v $PYLOVM2_DIR:/io -v $LOVM2_DIR/:/deps pylovm2-build

# Build pylovm2_stdlib with `maturin build`
echo "Build pylovm2_stdlib now ..."
docker run -ti --entrypoint "/io/build-pylovm2_stdlib.sh" -v $PYLOVM2_DIR/stdlib:/io -v $LOVM2_DIR/:/deps pylovm2-build
