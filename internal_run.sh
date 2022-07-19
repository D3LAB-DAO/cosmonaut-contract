#!/bin/bash

CONTRACT_DIR=$1
BASE_VOLUME_DIR=$2
WHICH_LESSON=$3
WHICH_CHAPTER=$4

sed -i '$d' ${BASE_VOLUME_DIR}/packages/cw20-tokens/Cargo.toml
echo cosmonaut-cw20 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw20\", features=[\"library\"]} >> ${BASE_VOLUME_DIR}/packages/cw20-tokens/Cargo.toml

sed -i '$d' ${BASE_VOLUME_DIR}/packages/cw721-spaceship/Cargo.toml
echo cosmonaut-cw721 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw721\", features=[\"library\"]} >> ${BASE_VOLUME_DIR}/packages/cw721-spaceship/Cargo.toml

sed -i '$d' ${BASE_VOLUME_DIR}/packages/main-contract/Cargo.toml
sed -i '$d' ${BASE_VOLUME_DIR}/packages/main-contract/Cargo.toml
sed -i '$d' ${BASE_VOLUME_DIR}/packages/main-contract/Cargo.toml

echo cosmonaut-cw20 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw20\", features=[\"library\"]} >> ${BASE_VOLUME_DIR}/packages/main-contract/Cargo.toml
echo cosmonaut-cw721 = {path= \"${CONTRACT_DIR}/contracts/cosmonaut-cw721\", features=[\"library\"]} >> ${BASE_VOLUME_DIR}/packages/main-contract/Cargo.toml
echo cosmonaut-main = {path=\"${CONTRACT_DIR}/contracts/cosmonaut-main\", features=[\"library\"]} >> ${BASE_VOLUME_DIR}/packages/main-contract/Cargo.toml

cargo run --manifest-path ${BASE_VOLUME_DIR}/lessons/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}/Cargo.toml $3 $4

