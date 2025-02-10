FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    gfortran libgfortran5 \
    && rm -rf /var/lib/apt/lists/* 

# Install RUST
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add path
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify the installation
RUN rustc --version && cargo --version

RUN apt-get update

WORKDIR /workspace

COPY . .

RUN cargo update
RUN cargo build --release

CMD ["/bin/bash"]