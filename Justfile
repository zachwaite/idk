test:
  #! /usr/bin/env bash
  cargo test -- --nocapture

graph rpgfile:
  #! /usr/bin/env bash
  ./target/debug/idk-graph dot {{ rpgfile }} > ~/Downloads/tmp-graph.gv
  dot -Tsvg /tmp/graph.gv > ~/Downloads/tmp-graph.svg

build:
  #! /usr/bin/env bash
  cargo build

deploy:
  #! /usr/bin/env bash
  cargo build --release
  cp ./target/release/libidk_nvim.so ~/.config/nvim/custom/rpgle.nvim/lua/idk.so
  cp ./target/release/idk-graph ~/.local/bin/idk-graph
  cp ./utils/idk-get ~/.local/bin/idk-get
  cp ./utils/idk-fmt ~/.local/bin/idk-fmt

