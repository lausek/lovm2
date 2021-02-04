FROM konstin2/maturin

# required for building stdlibs openssl
RUN yum update
RUN yum install perl-core

# lovm2 currently requires nightly
RUN rustup default nightly

ENTRYPOINT ["/bin/bash"]
