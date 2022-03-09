#!/usr/bin/env bash

set -e

cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
