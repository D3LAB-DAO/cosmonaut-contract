FROM rust:1.60
WORKDIR /home/app
COPY . .
ENTRYPOINT [ "cargo", "run",  "--"]
