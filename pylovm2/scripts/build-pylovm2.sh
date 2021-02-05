#!/bin/bash

# RUN INSIDE DOCKER CONTAINER!

case "$MATURIN_RELEASE" in
    "1")
        # Build project with correct environment variables
        env OPENSSL_LIB_DIR=/usr/local/lib64 \
            OPENSSL_INCLUDE_DIR=/usr/local/include \
            OPENSSL_STATIC=yes \
            maturin publish
        ;;

    *)
        # Build project with correct environment variables
        env OPENSSL_LIB_DIR=/usr/local/lib64 \
            OPENSSL_INCLUDE_DIR=/usr/local/include \
            OPENSSL_STATIC=yes \
            maturin build --release
        ;;
esac
