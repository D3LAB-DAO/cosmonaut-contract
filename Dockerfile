FROM rust:1.60

RUN apt-get update
RUN apt-get install vim -y
RUN ln -s /home/app/base/registry /usr/local/cargo/registry

WORKDIR /home/app
COPY . .
