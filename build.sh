#!/bin/sh

RUSTFLAGS="-Ctarget-cpu=native -Clink-args=-nostartfiles -Clink-args=-Wl,-n,-N,--no-dynamic-linker,--no-pie,--build-id=none" cargo build --release
