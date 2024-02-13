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

# stm
$cwd/target/release/cargo-embassy embassy init test-stm32g0 --chip stm32g031k8
$cwd/target/release/cargo-embassy embassy init test-stm32g4 --chip stm32g431rb --panic-handler reset

# nrf
$cwd/target/release/cargo-embassy embassy init test-nrf52840 --chip nrf52840
$cwd/target/release/cargo-embassy embassy init test-nrf52832 --chip nrf52832-xxab --softdevice s132

# compile
cd test-stm32g0; cargo build; cargo build --no-default-features --release
cd ../test-stm32g4; cargo build; cargo build --no-default-features --release
cd ../test-nrf52840; cargo build; cargo build --no-default-features --release
cd ../test-nrf52832; cargo build; cargo build --no-default-features --release

# clean up
cd ../..
# rm -r ci
