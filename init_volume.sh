#!/bin/bash

docker volume create base-volume

docker container create --name temp -v base-volume:/data busybox

docker cp ./packages temp:/data
docker cp ./answers temp:/data
docker cp ./lessons temp:/data
docker cp ./env/registry temp:/data

docker rm temp

