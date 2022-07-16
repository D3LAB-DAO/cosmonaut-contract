#!/bin/bash

echo $(pwd)

docker run --rm -it \
-v $(pwd)/$1/contracts:/home/app/contracts \
-v  $(pwd)/packages:/home/app/packages \
-v $(pwd)/answers:/home/app/answers \
-v $(pwd)/$1/target:/home/app/target \
-v $(pwd)/lessons:/home/app/lessons \
cosmonaut:1.0.0 cargo run -p lesson$2 -- $2
