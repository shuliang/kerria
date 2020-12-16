ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

LABEL version=1.0
WORKDIR /src-root

RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config

# required for auto-reload in development only.
RUN cargo install systemfd cargo-watch

# install movine for database migrations
RUN apt-get install -y libsqlite3-dev wait-for-it
RUN cargo install movine

ENTRYPOINT ["wait-for-it", "db:3306", "--", "./scripts/run_dev.sh"]
