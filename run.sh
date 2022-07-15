#!/bin/bash

rm -r ./target
docker run --rm -v $(pwd)/contracts:/home/app/contracts cosmonaut:1.0.0 $1

