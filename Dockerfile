FROM ubuntu:22.04

ENV PATH="${PATH}:/root/.cargo/bin"

RUN apt-get -y update && \
    apt-get -y install bison flex git curl libglib2.0-dev libfdt-dev \
        libpixman-1-dev zlib1g-dev ninja-build build-essential python3 python3-pip python3-venv && \
    python3 -m pip install sphinx sphinx_rtd_theme && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

COPY . /qemu-rs

WORKDIR /qemu-rs

RUN cargo build -r && \
    cargo run -r --bin tracer -- -a /bin/ls -- -lah

