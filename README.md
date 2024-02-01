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

This utility will also create the cargo project, so wherever you would normally run `cargo new ...`, run:

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
cargo embassy init my_project --chip stm32g031k8
```

That's it! You can `cargo flash` or `cargo run --features=defmt`.

**Create a new Embassy project for the NRF52840:**
```sh
cargo embassy init my_project --chip nrf52840
```

**Create a new Embassy project for the NRF52832_xxAA and Softdevice S132**
```sh
cargo embassy init my_project --chip nrf52832_xxAA --softdevice s132
```
