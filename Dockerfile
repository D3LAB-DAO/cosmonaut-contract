FROM rust:1.61

RUN apt-get update
RUN apt-get install vim -y

WORKDIR /workspace
COPY . .
