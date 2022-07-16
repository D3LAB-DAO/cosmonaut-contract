#!/bin/bash

echo $(pwd)

docker run --rm -it \
-v $(pwd)/$1/contracts:/home/app/contracts \
-v $(pwd)/packages:/home/app/packages \
-v $(pwd)/answers:/home/app/answers \
-v $(pwd)/$1/target/lesson$2:/home/app/lessons/lesson$2/target \
-v $(pwd)/env/registry:/usr/local/cargo/registry \
cosmonaut:1.0.0 cargo run --manifest-path lessons/lesson$2/Cargo.toml -- $2
