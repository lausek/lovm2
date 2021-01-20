#!/bin/bash

PYLOVM2_DIR=$(dirname `realpath $0`)
cd $PYLOVM2_DIR

docker build -t pylovm2-test -f test.Dockerfile .

docker run -ti \
    -v $PYLOVM2_DIR/target/wheels:/deps/pylovm2 \
    -v $PYLOVM2_DIR/stdlib/target/wheels:/deps/pylovm2_stdlib \
    pylovm2-test
