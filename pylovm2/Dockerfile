FROM konstin2/maturin

# required for building stdlibs openssl
RUN yum update
RUN yum install perl-core

# currently required
RUN rustup default nightly

ENTRYPOINT ["/bin/bash"]
