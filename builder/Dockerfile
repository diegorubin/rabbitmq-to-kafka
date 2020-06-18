FROM fedora:33
LABEL maintainer="Diego Rubin <rubin.diego@gmail.com>"

RUN dnf -y install gcc \
      make \
      cmake \
      gcc-c++

RUN dnf clean all

ARG CHANNEL="nightly"
ENV RUSTUP_VER="1.21.1" \
    RUST_ARCH="x86_64-unknown-linux-gnu"
RUN curl "https://static.rust-lang.org/rustup/archive/${RUSTUP_VER}/${RUST_ARCH}/rustup-init" -o rustup-init && \
    chmod +x rustup-init && \
    ./rustup-init -y --default-toolchain ${CHANNEL} --profile minimal && \
    rm rustup-init

ENV PATH=/usr/local/bin:/root/.cargo/bin:$PATH \
    PKG_CONFIG_PATH=/usr/lib/pkgconfig

WORKDIR /volume
