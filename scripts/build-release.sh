#! /bin/bash
set -e

rustup target add x86_64-unknown-linux-musl

cargo build --release --target=x86_64-unknown-linux-musl

ls -lh target/*/release/simple-portal
