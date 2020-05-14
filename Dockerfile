# This file is largely based on https://github.com/rust-lang/rust-log-analyzer/blob/master/Dockerfile
# Which is licensed under the MIT License https://github.com/rust-lang/rust-log-analyzer/blob/master/LICENSE

FROM ubuntu:bionic AS build

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates \
    curl \
    build-essential \
    pkg-config \
    libssl-dev

    # Install the currently pinned toolchain with rustup
RUN curl https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init >/tmp/rustup-init && \
    chmod +x /tmp/rustup-init && \
    /tmp/rustup-init -y --no-modify-path --default-toolchain stable
ENV PATH=/root/.cargo/bin:$PATH

# Build the dependencies in a separate step to avoid rebuilding all of them
# every time the source code changes. This takes advantage of Docker's layer
# caching, and it works by copying the Cargo.{toml,lock} with dummy source code
# and doing a full build with it.
WORKDIR /tmp/source
COPY Cargo.lock Cargo.toml /tmp/source/
RUN mkdir -p /tmp/source/src && \
    echo "fn main() {}" > /tmp/source/src/main.rs
RUN cargo fetch
RUN cargo build --release

# Dependencies are now cached, copy the actual source code and do another full
# build. The touch on all the .rs files is needed, otherwise cargo assumes the
# source code didn't change thanks to mtime weirdness.
RUN rm -rf /tmp/source/src
COPY src /tmp/source/src
RUN find -name "*.rs" -exec touch {} \; && cargo build --release


##################
#  Output image  #
##################

FROM ubuntu:bionic AS binary

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates

COPY --from=build /tmp/source/target/release/crates-io-ops-bot /usr/local/bin/

CMD ["crates-io-ops-bot"]