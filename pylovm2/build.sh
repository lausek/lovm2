#!/bin/bash

PYLOVM2_DIR=$(dirname `realpath $0`)
cd $PYLOVM2_DIR

pushd $PYLOVM2_DIR/../
LOVM2_DIR=$(pwd)
popd

MATURIN_RELEASE=0

case "$1" in
    "release")
        MATURIN_RELEASE=1
        ;;
esac

echo "Building container now ..."
docker build -t pylovm2-build .

# Build pylovm2 with `maturin build`
echo "Building pylovm2 now ..."
docker run -ti --entrypoint "/io/scripts/build-pylovm2.sh" \
    -v $PYLOVM2_DIR:/io -v $LOVM2_DIR/:/deps \
    -e MATURIN_RELEASE=$MATURIN_RELEASE \
    pylovm2-build
