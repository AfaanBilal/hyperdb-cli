#
# HyperDB CLI
#
# CLI for HyperDB (https://github.com/AfaanBilal/hyperdb)
#
# @author Afaan Bilal
# @link   https://afaan.dev
#

#
# Stage 1 (Build)
#

FROM rust:1.66-slim-buster AS build

WORKDIR /hyperdb-cli

COPY . .

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:buster-slim

WORKDIR /hyperdb-cli

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y pkg-config libssl-dev
COPY --from=build /hyperdb-cli/target/release/hyperdb-cli ./hyperdb-cli

# And away we go...
ENTRYPOINT [ "./hyperdb-cli" ]
