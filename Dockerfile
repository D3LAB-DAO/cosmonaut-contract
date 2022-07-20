FROM rust:1.60

RUN apt-get update
RUN apt-get install vim -y

ARG CARGO_REGISTRY_DIR
RUN ln -s ${CARGO_REGISTRY_DIR} /usr/local/cargo/registry

WORKDIR /home/app
COPY . .
