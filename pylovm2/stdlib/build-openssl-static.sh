#!/bin/bash

# RUN INSIDE DOCKER CONTAINER!

# Include OpenSSL statically in stdlib, because it is not supported by manylinux.
# Based on this explanation: https://github.com/sfackler/rust-openssl/issues/183#issuecomment-81686336

# Build OpenSSL with PIC support: https://github.com/rust-lang/cargo/issues/713#issuecomment-59597433
OPENSSL_RELEASE=OpenSSL_1_0_1-stable
pushd /tmp

git clone git://git.openssl.org/openssl.git --depth 1 --branch $OPENSSL_RELEASE
cd openssl
make clean
./config shared no-idea no-mdc2 no-rc5 no-ssl2 no-ssl3 \
    enable-ec_nistp_64_gcc_128 --prefix=/usr/local --openssldir=/usr/local/ssl
make depend
make
make install

popd

# Build project with correct environment variables
env OPENSSL_LIB_DIR=/usr/local/lib64 \
    OPENSSL_INCLUDE_DIR=/usr/local/include \
    OPENSSL_STATIC=yes \
    maturin build
