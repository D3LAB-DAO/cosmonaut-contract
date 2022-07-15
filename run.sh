#!/bin/bash

if [ -d "./target" ]
then
  rm -r ./target
fi

docker run --rm -v $(pwd)/contracts:/home/app/contracts cosmonaut:1.0.0 $1

