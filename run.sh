docker run --rm -it \
-v base-volume:/home/app/base \
-v user1:/home/app/user1 \
-e BASE_VOLUME_DIR=/home/app/base \
cosmonaut:1.0.0 bash
