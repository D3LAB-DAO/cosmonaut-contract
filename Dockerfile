FROM rust:1.60

RUN ln -s /home/app/base/registry /usr/local/cargo/registry

WORKDIR /home/app
COPY . .
