FROM rust:buster as builder

RUN apt update && apt install -y libssl-dev

WORKDIR /usr/src/jmserver

COPY Cargo.toml ./
COPY src/ src/
COPY templates/ templates/

RUN cargo build --release

FROM debian:buster

COPY --from=builder /usr/src/jmserver/target/release/jmserver /usr/bin

RUN apt update && apt install -y libssl1.1 dumb-init curl

VOLUME ["/data"]

ENTRYPOINT ["/usr/bin/dumb-init", "--", "/usr/bin/jmserver", "--config", "/data/config.toml"]