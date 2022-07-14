#!/bin/bash

docker run --rm -it -v ${HOME}/.cargo/registry:/usr/local/cargo/registry cosmonaut:1.0.0 bash
