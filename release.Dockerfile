ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/kerria \
    /usr/local/bin/
CMD ["/usr/local/bin/kerria"]
