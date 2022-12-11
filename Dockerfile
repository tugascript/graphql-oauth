# generate a rust image with the latest stable version of rust
FROM rust:latest AS builder

# create app directory
WORKDIR /app

# copy the source code to the working directory
COPY . .

# build the project
RUN cargo build --release

# generate a new image with the binary
FROM debian:buster-slim

# copy the binary from the builder image
COPY --from=builder /app/target/release/graphql-local-oauth /usr/local/bin

# run the binary
WORKDIR /usr/local/bin
CMD ["graphql-local-oauth"]