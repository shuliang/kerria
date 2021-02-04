ARG BASE_IMAGE=ekidd/rust-musl-builder:1.49.0

FROM ${BASE_IMAGE} AS builder

USER root

RUN mkdir -p ~/.cargo/ && touch ~/.cargo/config && printf "\n[source.crates-io]\nreplace-with = 'tuna'\n\n[source.tuna]\nregistry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"" >> ~/.cargo/config

WORKDIR /home/rust/src
COPY . ./

ARG DATABASE_URL
RUN cargo build --release

# FROM alpine:latest
# RUN apk --no-cache add libressl-dev && apk --no-cache add ca-certificates
FROM shuliang/alpine-ssl

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/kerria \
    /usr/local/bin/

ENV RUST_LOG=info
CMD ["/usr/local/bin/kerria"]
