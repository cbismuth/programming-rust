#!/usr/bin/env bash

set -e

export RUST_BACKTRACE="full"

cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
