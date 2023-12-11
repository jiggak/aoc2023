#!/bin/bash

cargo build --release

CMD="${PWD}/target/release/day5"
CMD2="${PWD}/target/release/day5 -p2"
source ../tests/run.sh

run_day_tests day5