#!/usr/bin/env bash

set -o nounset
set -o errexit
set -o pipefail

cd "$(dirname "$0")"

rm -f /tmp/spell.sqlite

# build spell.wasm
marine build --release
cp target/wasm32-wasi/release/spell.wasm tests_artifacts/

if [ ! -f "tests_artifacts/sqlite3.wasm" ]; then
  # download SQLite 3
  curl -L https://github.com/fluencelabs/sqlite/releases/download/v0.15.0_w/sqlite3.wasm -o tests_artifacts/sqlite3.wasm
fi

# run tests
cargo test --release -- --nocapture --test-threads 1
