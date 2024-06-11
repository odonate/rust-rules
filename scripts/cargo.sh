#!/bin/bash
export LD_LIBRARY_PATH=/opt/rust-rules/toolchain/1.0/opt/rust/1.78.0/lib:$LD_LIBRARY_PATH
export RUSTC=/opt/rust-rules/toolchain/1.0/opt/rust/1.78.0/rustc
export RUST_LIB_PATH=/opt/rust-rules/toolchain/1.0/opt/rust/1.78.0/lib
export RUSTFLAGS="-L /opt/rust-rules/toolchain/1.0/opt/rust/1.78.0/stdlib/lib"
export CARGO_TARGET_DIR=plz-out/gen/target
exec /opt/rust-rules/toolchain/1.0/opt/rust/1.78.0/cargo "$@"
