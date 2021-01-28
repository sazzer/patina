# Build the code first
FROM rust:slim-buster AS builder

WORKDIR /patina

# First install any needed system dependencies
RUN rustc --version --verbose
RUN apt update -y && apt install -y pkg-config libssl-dev libclang-dev clang

# Download and compile the rust dependencies
RUN USER=patina cargo init --bin --name patina .
COPY Cargo.lock Cargo.toml /patina/

RUN USER=patina cargo init --lib --name=patina-testdatabase crates/testdatabase
COPY crates/testdatabase/Cargo.lock crates/testdatabase/Cargo.toml /patina/crates/testdatabase/

RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /patina/target -name "*patina*" | xargs rm -rf

# Now build the actual app
RUN rm -rf /patina/src
COPY src /patina/src

RUN rm -rf /patina/crates/testdatabase/src
COPY crates/testdatabase/src /patina/crates/testdatabase/src

COPY migrations /patina/migrations

RUN cargo build --release

# Next build a container with the build artifact but no code
FROM debian:buster-slim

RUN apt update -y && apt install -y openssl ca-certificates wget

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /patina

COPY --from=builder /patina/target/release/patina /patina/patina
COPY docker/start.sh /patina

ENV PORT=8000
ENV RUST_LOG=debug

CMD ["/patina/start.sh"]

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD wget -q http://localhost:$PORT/health
