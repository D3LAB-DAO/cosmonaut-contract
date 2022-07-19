BASE_VOLUME_DIR=/home/app/base

docker run --rm -it \
-v base-volume:/home/app/base \
-v user1:/home/app/user1 \
-e BASE_VOLUME_DIR=$BASE_VOLUME_DIR \
cosmonaut:1.0.0 cargo run --manifest-path $BASE_VOLUME_DIR/lessons/lesson$1/chapter$2/Cargo.toml $1 $2
