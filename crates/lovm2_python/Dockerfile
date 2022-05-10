FROM konstin2/maturin:v0.12.10
ENV LOVM2_PYTHON_DIR="/build/crates/lovm2_python"

# required for building stdlibs openssl
RUN yum update && yum install -y perl-core

# build openssl with static linkage support
COPY ./scripts/build-openssl.sh /tmp/build-openssl.sh
RUN /tmp/build-openssl.sh

# lovm2 currently requires nightly
RUN rustup default nightly

ENTRYPOINT ["/bin/bash"]
