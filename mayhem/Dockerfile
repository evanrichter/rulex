# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src
RUN cd rulex-lib/fuzz/ && cargo +nightly fuzz build

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /src/rulex-lib/fuzz//target/x86_64-unknown-linux-gnu/release/parse_and_compile /
