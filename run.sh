BASE_VOLUME_DIR=/home/app/base
USER_VOLUME_NAME=user
USER_CONTRACT_DIR=/home/app/user1

docker run --rm -it \
  -v base-volume:$BASE_VOLUME_DIR \
  -v $USER_VOLUME_NAME:$USER_CONTRACT_DIR \
  -e BASE_VOLUME_DIR=$BASE_VOLUME_DIR \
  cosmonaut:1.0.0 cargo run --manifest-path $BASE_VOLUME_DIR/lessons/lesson$1/chapter$2/Cargo.toml $1 $2
