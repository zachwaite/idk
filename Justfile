#! /usr/bin/env just

test:
  cargo test -- --nocapture

build:
  cargo build

release:
  cargo build --release
  cp ./target/release/libidk_nvim.so ~/.config/nvim/custom/rpgle.nvim/lua/idk.so
  cp ./target/release/idk-graph ~/.local/bin/idk-graph

