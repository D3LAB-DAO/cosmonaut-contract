#!/bin/bash

USER_ID=$1
WHICH_LESSON=$2
WHICH_CHAPTER=$3

PROJ_BASE=/workspace/cargo-projects
SKELETON_PATH=/workspace/base/skeleton/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}/contracts

if [ -d $SKELETON_PATH ]; then
    mkdir -p ${PROJ_BASE}/cosm/${USER_ID}/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}
    cp -r ${SKELETON_PATH} ${PROJ_BASE}/cosm/${USER_ID}/lesson${WHICH_LESSON}/chapter${WHICH_CHAPTER}
else
    echo "Lesson or chapter not found"
fi
