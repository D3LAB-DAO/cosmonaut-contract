FROM rust:1.60

VOLUME [ "${HOME}/.cargo/bin" ]
VOLUME [ "./target" ]

WORKDIR /home/app
ENTRYPOINT [ "cargo", "run",  "--"]