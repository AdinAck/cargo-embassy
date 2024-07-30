#!/bin/bash

set -euxo pipefail

# build cargo-embassy
cargo build --release

test_dir="/tmp/ci"

# create test directory
if [ -d $test_dir ]; then
    rm -r $test_dir
fi

cwd=`pwd`

mkdir $test_dir
cd $test_dir

# generation
$cwd/target/release/cargo-embassy embassy init test-esp32c3 --chip esp32c3
$cwd/target/release/cargo-embassy embassy init test-esp32s3 --chip esp32s3

# esp toolchain
if [ "${1-""}" = "--install-esp" ]; then
    cargo install espup
    espup install
else
    echo "Skipping ESP toolchain installation."
fi

. $HOME/export-esp.sh

# compile
cd ../test-esp32c3; cargo build --release
cd ../test-esp32s3; cargo build --release

# clean up
cd ../..
# rm -r ci
