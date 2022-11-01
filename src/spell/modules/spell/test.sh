#!/usr/bin/env sh

set -o nounset
set -o errexit
set -o pipefail

cd "$(dirname "$0")"

marine build --release
cp target/wasm32-wasi/release/spell.wasm tests_artifacts
cargo test --release -- --nocapture
