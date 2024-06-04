#! /usr/bin/env just

test:
  cargo test -- --nocapture

build:
  cargo build

