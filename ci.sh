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

# demo memory.x file
echo "MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K

  /* These values correspond to the NRF52840 with Softdevices S140 7.3.0 */
  /*
     FLASH : ORIGIN = 0x00027000, LENGTH = 868K
     RAM : ORIGIN = 0x20020000, LENGTH = 128K
  */
}" > test-nrf52840/memory.x

# compile
cd test-stm32g0; cargo build; cargo build --features=defmt
cd ../test-stm32g4; cargo build; cargo build --features=defmt
cd ../test-nrf52840; cargo build; cargo build --features=defmt

# clean up
cd ../..
rm -r ci
