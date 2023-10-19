#! /bin/bash
set -e

rustup target add x86_64-unknown-linux-musl

cargo build --release --target=x86_64-unknown-linux-musl

echo "Outputs:"
ls -lh target/*/release/simple-portal | awk '{print $5,$(NF)}'

mkdir -p release
ls target/*/release/simple-portal | awk -F '/' '{printf "%s release/%s-%s",$0,$(NF),$2}' | xargs cp
gzip release/*
