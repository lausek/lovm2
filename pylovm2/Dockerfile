FROM quay.io/pypa/manylinux1_x86_64

RUN useradd -m builder
USER builder

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH="/home/builder/.cargo/bin:$PATH"

COPY . /app
WORKDIR /app

USER 0
RUN chown -R builder:builder /app
USER builder

ENTRYPOINT ["/bin/bash"]
