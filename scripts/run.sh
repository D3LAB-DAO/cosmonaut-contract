#!/bin/bash

USER_ID=$1
WHICH_LESSON=$2
WHICH_CHAPTER=$3

BASE_DIR=/workspace/cargo-projects
BASE_VOLUME_DIR=/workspace/base

USER_CONTRACT_DIR=${BASE_DIR}/cosm/${USER_ID}/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}

if [ -d $USER_CONTRACT_DIR ]; then
  docker run --rm \
    -v base-volume:$BASE_VOLUME_DIR \
    -v cosmonaut-api_cosmproj:$BASE_DIR \
    -e BASE_VOLUME_DIR=$BASE_VOLUME_DIR \
    cosmonaut-contract:1.0.0 scripts/internal_run.sh $USER_CONTRACT_DIR $BASE_VOLUME_DIR $WHICH_LESSON $WHICH_CHAPTER
else
  echo "contract not found"
fi
