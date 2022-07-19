FROM rust:1.60

ARG CARGO_REGISTRY_DIR
RUN ln -s $CARGO_REGISTRY_DIR /usr/local/cargo/registry

WORKDIR /home/app
COPY . .
