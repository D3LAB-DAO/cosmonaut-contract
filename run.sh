#!/bin/bash

docker run --rm -it \
-v $(pwd)/users/$1/contracts:/home/app/contracts \
-v $(pwd)/packages:/home/app/packages \
-v $(pwd)/answers:/home/app/answers \
-v $(pwd)/users/$1/targets/lesson$2:/home/app/lessons/lesson$2/target \
-v $(pwd)/env/registry:/usr/local/cargo/registry \
cosmonaut:1.0.0 cargo run --manifest-path lessons/lesson$2/Cargo.toml -- $2
