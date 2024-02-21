#!/usr/bin/env bash

set -o nounset
set -o errexit
set -o pipefail

cd "$(dirname "$0")"

rm -f /storage/spell.sqlite

# build spell.wasm
marine build --release
cp ../target/wasm32-wasi/release/spell.wasm tests_artifacts/
mkdir -p tests_artifacts/tmp

if [[ ! -f "tests_artifacts/sqlite3.wasm" ]]; then
  # download SQLite 3
  curl -L https://github.com/fluencelabs/sqlite/releases/download/sqlite-wasm-v0.18.2/sqlite3.wasm -o tests_artifacts/sqlite3.wasm
fi

# run tests
cargo nextest run --release --no-fail-fast --nocapture
