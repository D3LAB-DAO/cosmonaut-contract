#!/bin/bash

USER_ID=$1
WHICH_LESSON=$2
WHICH_CHAPTER=$3

PROJ_BASE=/workspace/cargo-projects

mkdir -p ${PROJ_BASE}/cosm/${USER_ID}/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}
cp -r /workspace/base/skeleton/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}/contracts ${PROJ_BASE}/cosm/${USER_ID}/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}
