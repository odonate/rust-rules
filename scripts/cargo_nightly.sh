#!/bin/bash
export LD_LIBRARY_PATH=~/rust-rules/plz-out/gen/third_party/rust/rust-nightly-x86_64-unknown-linux-gnu/rust-std-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH
export RUSTC=~/rust-rules/plz-out/bin/third_party/rust/rust-nightly-x86_64-unknown-linux-gnu/rustc/bin/rustc
export RUST_LIB_PATH=~/rust-rules/plz-out/gen/third_party/rust/rust-nightly-x86_64-unknown-linux-gnu/rust-std-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib
export RUSTFLAGS="-L $RUST_LIB_PATH"
export CARGO_TARGET_DIR=~/rust-rules/plz-out/gen/nightly/target
exec plz run //third_party/rust:toolchain_nightly_cargo -- "$@"
