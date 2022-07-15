#!/bin/bash

if [ -d "./target" ]
then
  rm -r ./target
fi

docker run --rm \
-v $(pwd)/$1/contracts:/home/app/contracts \
-v  $(pwd)/packages:/home/app/packages \
-v $(pwd)/answers:/home/app/answers \
cosmonaut:1.0.0 $2

