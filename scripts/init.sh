#!/bin/bash

CARGO_REGISTRY_DIR=/workspace/base/registry

cd ..

docker build . --build-arg CARGO_REGISTRY_DIR=$CARGO_REGISTRY_DIR -t cosmonaut-contract:1.0.0
docker volume create base-volume

docker container create --name temp -v base-volume:/data busybox

docker cp ./packages temp:/data
docker cp ./answers temp:/data
docker cp ./lessons temp:/data
docker cp ./skeleton temp:/data
docker cp ./env/registry temp:/data

docker rm temp
