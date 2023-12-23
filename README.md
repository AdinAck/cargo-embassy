# cargo-embassy

Get up and running with Embassy in seconds.

# Features
- Supports STM32* and NRF*
- Generates project structure
  - Toolchain
  - Probing
  - Dependencies
  - Profiles
  - Formatting

# TODO

Refer to the tracking issues for propsed changes.

# Installation

Simply install this repo as a cargo utility:

```sh
cargo install --git https://github.com/adinack/cargo-embassy
```

# Usage

This utility will also create the cargo project, so wherever you would normall run `cargo new ...`, run:

```sh
cargo embassy init {project_name} args...
```

You can see how the `init` command works with:

```sh
cargo embassy init --help
```

# Examples

**Create a new Embassy project for the STM32G031K8:**
```sh
cargo embassy init my_project --family stm32 --chip stm32g031k8 --target thumbv6
```

That's it! You can `cargo flash` or `cargo run --features=defmt`.

**Create a new Embassy project for the STM32G031K8 with a specific Embassy version:**
```sh
cargo embassy init my_project --family stm32 --chip stm32g031k8 --target thumbv6 --commit 5bc75578260f4c644cc060e6458a05d7fc0ffb41
```

**Create a new Embassy project for the NRF52840:**
```sh
cargo embassy init my_project --family nrf --chip nrf52840 --target thumbv7e
```

Update `memory.x` appropriately.
