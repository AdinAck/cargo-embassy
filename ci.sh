#!/bin/bash

set -euxo pipefail

# build cargo-embassy
cargo build --release -q

# create test directory
if [ -d "ci" ]; then
    rm -r "ci"
fi

mkdir ci
cd ci

# generation

# stm
../target/release/cargo-embassy embassy init test-stm32g0 --chip stm32g031k8 > /dev/null
../target/release/cargo-embassy embassy init test-stm32g4 --chip stm32g431rb --panic-handler reset > /dev/null

# nrf
../target/release/cargo-embassy embassy init test-nrf52840 --chip nrf52840 > /dev/null

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
cd test-stm32g0; cargo build -q; cargo build --features=defmt -q
cd ../test-stm32g4; cargo build -q; cargo build --features=defmt -q
cd ../test-nrf52840; cargo build -q; cargo build --features=defmt -q

# clean up
cd ../..
rm -r ci
