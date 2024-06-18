#! /usr/bin/env just

test:
  cargo test -- --nocapture

build:
  cargo build

deploy:
  cargo build --release
  cp ./target/release/libidk_nvim.so ~/.config/nvim/custom/rpgle.nvim/lua/idk.so
  cp ./target/release/idk-graph ~/.local/bin/idk-graph
  cp ./utils/idk-get ~/.local/bin/idk-get
  cp ./utils/idk-fmt ~/.local/bin/idk-fmt

