#!/bin/bash

CARGO_REGISTRY_DIR=/home/app/base/registry
CONTRACT_DIR=/home/app/user

# delete blank after -i for arm64 architecture
# CONTRACT_DIR must be absolute path
sed -i '' '$d' packages/cw20-tokens/Cargo.toml
echo cosmonaut-cw20 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw20\", features=[\"library\"]} >> packages/cw20-tokens/Cargo.toml

sed -i '' '$d' packages/cw721-spaceship/Cargo.toml
echo cosmonaut-cw721 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw721\", features=[\"library\"]} >> packages/cw721-spaceship/Cargo.toml

sed -i '' '$d' packages/main-contract/Cargo.toml
sed -i '' '$d' packages/main-contract/Cargo.toml
sed -i '' '$d' packages/main-contract/Cargo.toml

echo cosmonaut-cw20 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw20\", features=[\"library\"]} >> packages/main-contract/Cargo.toml
echo cosmonaut-cw721 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw721\", features=[\"library\"]} >> packages/main-contract/Cargo.toml
echo cosmonaut-main = {path=\"${CONTRACT_DIR}/contracts/cosmonaut-main\", features=[\"library\"]} >> packages/main-contract/Cargo.toml

docker build . --build-arg CARGO_REGISTRY_DIR=$CARGO_REGISTRY_DIR -t cosmonaut:1.0.0
docker volume create base-volume

docker container create --name temp -v base-volume:/data busybox

docker cp ./packages temp:/data
docker cp ./answers temp:/data
docker cp ./lessons temp:/data
docker cp ./env/registry temp:/data

docker rm temp
