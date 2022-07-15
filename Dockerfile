FROM rust:1.60

WORKDIR /home/app
COPY . .
RUN mv env/registry /usr/local/cargo

ENTRYPOINT [ "cargo", "run",  "--"]
