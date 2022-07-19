#!/bin/bash

CARGO_REGISTRY_DIR=/home/app/base/registry

docker build . --build-arg CARGO_REGISTRY_DIR=$CARGO_REGISTRY_DIR -t cosmonaut:1.0.0
docker volume create base-volume

docker container create --name temp -v base-volume:/data busybox

docker cp ./packages temp:/data
docker cp ./answers temp:/data
docker cp ./lessons temp:/data
docker cp ./env/registry temp:/data

docker rm temp
