#!/bin/sh

plz build //third_party/rust:toolchain_deb
plz query output //third_party/rust/... | grep .deb | while read -r line ; do
    sudo dpkg -i $line
done
sudo chmod +x /opt/rust-rules/toolchain/1.0/bin/*
