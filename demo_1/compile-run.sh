#!/bin/bash
EXEC_FILE="sample_game"
cargo build && ./target/debug/"$EXEC_FILE"
