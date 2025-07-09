#!/bin/bash

# a~gのいずれかを取得
PROBLEM=$(echo $1 | tr '[:upper:]' '[:lower:]')
cargo test --test $PROBLEM