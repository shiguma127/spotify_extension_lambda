#!/bin/bash -x
# amazonlinux2 の docker 環境で build する。

# start up docker container
docker compose build
docker compose up -d

# build
docker exec spotify_extension_lambda-builder cargo build --release
docker exec spotify_extension_lambda-builder mv ./target/release/spotify_extension_lambda bootstrap
docker exec spotify_extension_lambda-builder strip --strip-all bootstrap && size bootstrap && ldd bootstrap
docker exec spotify_extension_lambda-builder chmod 755 bootstrap
docker exec spotify_extension_lambda-builder zip bootstrap.zip bootstrap

echo "✨✨✨ done ✨✨✨"

