#!/bin/bash
export LD_LIBRARY_PATH=~/rust-rules/plz-out/gen/third_party/rust/rust-1.78.0-x86_64-unknown-linux-gnu/rust-std-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH
export RUST_LIB_PATH=~/rust-rules/plz-out/gen/third_party/rust/rust-1.78.0-x86_64-unknown-linux-gnu/rust-std-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib
export RUSTFLAGS="-L $RUST_LIB_PATH"
exec plz run //third_party/rust:toolchain_rustc -- "$@"
