#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# build spell.wasm
marine build --release

# copy .wasm to artifacts
mkdir -p artifacts
cp target/wasm32-wasi/release/spell.wasm artifacts/

# download SQLite 3 to use in tests
curl -L https://github.com/fluencelabs/sqlite/releases/download/v0.15.0_w/sqlite3.wasm -o artifacts/sqlite3.wasm

cd ./artifacts
tar --exclude="spell.tar.gz" -f spell.tar.gz -zcv ./*
