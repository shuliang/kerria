# FROM rust:1.49.0-slim as builder

# LABEL version=1.0

# WORKDIR /src-root

# RUN mkdir -p ~/.cargo/ && touch ~/.cargo/config && printf "\n[source.crates-io]\nreplace-with = 'tuna'\n\n[source.tuna]\nregistry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"" >> ~/.cargo/config

# RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config \
#     && apt-get install -y clang llvm-dev libclang-dev \
#     && apt-get install -y libsqlite3-dev wait-for-it \
#     && rm -rf /var/lib/{apt,dpkg,cache,log}/ \
#     && cargo install systemfd cargo-watch sqlx-cli

FROM shuliang/kerria-dev as builder
ENTRYPOINT ["wait-for-it", "db:3306", "--", "./scripts/run_dev.sh"]
