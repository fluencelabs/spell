#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# build spell.wasm
marine build --release

# copy .wasm to artifacts
mkdir -p artifacts
cp ../target/wasm32-wasi/release/spell.wasm artifacts/

if [[ ! -f "artifacts/sqlite3.wasm" ]]; then
  # download SQLite 3
  curl -L https://github.com/fluencelabs/sqlite/releases/download/v0.17.1_w/sqlite3.wasm -o artifacts/sqlite3.wasm
fi

cd ./artifacts
tar --exclude="spell.tar.gz" -f spell.tar.gz -zcv ./*
mkdir -p ../../spell-distro/spell-service
cp -v spell.wasm sqlite3.wasm Config.toml ../../spell-distro/spell-service/
