#!/usr/bin/env sh
cargo fix --allow-staged
cargo fmt
cargo test
