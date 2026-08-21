test:
  cargo test --workspace

deploy-idk-nvim:
  #! /usr/bin/env bash
  cargo build -p idk-nvim --release
  cp ./target/release/libidk_nvim.so ./idk.nvim/lua/libidk.so

deploy-idk-get:
  #! /usr/bin/env bash
  cp ./utils/idk-get ~/.local/bin/idk-get
  cp ./utils/idk-fmt ~/.local/bin/idk-fmt

debug-idk-nvim:
  #! /usr/bin/env bash
  cargo build --package idk-nvim
  cp ./target/debug/libidk_nvim.so ./idk.nvim/lua/libidk.so

debug: debug-idk-nvim deploy-idk-get

