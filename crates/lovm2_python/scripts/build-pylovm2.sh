#!/bin/bash

# RUN INSIDE DOCKER CONTAINER!

pushd $LOVM2_PYTHON_DIR

# Build project with correct environment variables
export OPENSSL_LIB_DIR=/usr/local/lib64
export OPENSSL_INCLUDE_DIR=/usr/local/include
export OPENSSL_STATIC=yes

case "$MATURIN_RELEASE" in
    "1")
        maturin publish
        ;;

    *)
        maturin build --release
        ;;
esac
