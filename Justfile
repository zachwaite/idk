test:
  cargo test --workspace

deploy:
  #! /usr/bin/env bash
  cargo build --release
  cp ./target/release/libidk_nvim.so ./idk.nvim/lua/libidk.so

deploy-idk-get:
  #! /usr/bin/env bash
  cp ./utils/idk-get ~/.local/bin/idk-get
  cp ./utils/idk-fmt ~/.local/bin/idk-fmt

debug-idk-nvim:
  #! /usr/bin/env bash
  cargo build --package idk-nvim
  cp ./target/debug/libidk_nvim.so ./idk.nvim/lua/libidk.so

debug-idkpy:
  #! /usr/bin/env bash
  # idkpy
  cd ./crates/idkpy/ \
    && source ./venv/bin/activate \
    && maturin build \
    && cd ../../ \
    && cp ./target/wheels/*.whl ../test-idk/

debug: debug-idk-nvim debug-idkpy deploy-idk-get

