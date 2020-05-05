#!/usr/bin/env bash

mkdir lists

docker container run -d \
    --name words-web-local \
    -p 14000:5000 \
    -v $(realpath lists):/app/lists \
    raatex/words:latest