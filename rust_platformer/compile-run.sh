#!/bin/bash
EXEC_FILE="rust_platformer"
cargo build && ./target/debug/"$EXEC_FILE"
