# straknet-class-hash

[![Crates.io](https://img.shields.io/crates/v/class-hash.svg)](https://crates.io/crates/class-hash)

## Overview

Utility for getting starknet contract's class hashes from Scarb projects.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install class-hash`

## Usage

Run `class_hash get` for generating the class hashes of every contract in your project's scope.

To obtain `sierra` and `casm` class hashes, add the following entries in you `Scarb.toml` manifest file:

```
[lib]

[[target.starknet-contract]]
allowed-libfuncs-list.name = "experimental"
sierra = true
casm = true
```

You should see a similar output in the terminal:

![](images/example.png)

## License

Licensed under the MIT license.
