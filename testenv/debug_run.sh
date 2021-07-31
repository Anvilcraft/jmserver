#!/bin/sh
export DBURL="mysql://jensmemes:snens@127.0.0.1:3306/jensmemes"
export CDNURL="http://127.0.0.1:8082"
export LISTEN="127.0.0.1:8081"

export RUST_BACKTRACE=1

cargo run
