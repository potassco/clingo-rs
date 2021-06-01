# clingo-rs [![Build Status](https://travis-ci.org/potassco/clingo-rs.svg?branch=master)](https://travis-ci.org/potassco/clingo-rs) [![Latest Version](https://img.shields.io/crates/v/clingo.svg)](https://crates.io/crates/clingo) [![Rust Documentation](https://docs.rs/clingo/badge.svg)](https://docs.rs/clingo)

Rust bindings to the [clingo](https://github.com/potassco/clingo) library.
Clingo version 5.5.0.

## Requirements

- a c++14 conforming compiler
  - *at least* [gcc](https://gcc.gnu.org/) version 4.9
  - [clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
    provided by gcc 4.9 or libc++)

Per default the crate uses the clingo library via dynamic linking.
It is assumed that a clingo dynamic library is installed on the system.
You have to set the environment variable `CLINGO_LIBRARY_PATH`. For example:

```sh
export CLINGO_LIBRARY_PATH=/scratch/miniconda3/envs/test/lib
```

## Examples

```sh
cargo run --example=ast 0
cargo run --example=backend 0
cargo run --example=configuration
cargo run --example=control 0
cargo run --example=model 0
cargo run --example=propagator 0
cargo run --example=solve-async 0
cargo run --example=statistics 0
cargo run --example=symbol 0
cargo run --example=symbolic-atoms 0
cargo run --example=theory-atoms 0
cargo run --example=inject-terms 0
cargo run --example=version
```

## Features

### Using `derive` macro

The crate provides a derive macro to help easing the use of rust data types as facts.

In your `Cargo.toml` add:

```toml
[dependencies]
clingo = { version = "0.7.0-beta.1", features = ["derive"] }
```

In your source write:

```ignore
use clingo::ToSymbol;
use clingo::ClingoError;
use clingo::FactBase;

#[derive(ToSymbol)]
struct MyPoint {
    x: i32,
    y: i32,
}

let p = MyPoint{ x:4, y:2 };
let fb = FactBase::new();
fb.insert(p);
```

The macro performs a conversion to snake case. This means the corresponing fact for `MyPoint{x:4,y:2}` is `my_point(4,2)`.

### Using `dl-theory`

You have to set the environment variable `CLINGO_DL_LIBRARY_PATH`. For example:

```sh
export CLINGO_DL_LIBRARY_PATH=/scratch/miniconda3/envs/test/lib
```

The recommended way to use the optional dl-theory feature is as
follows.

```toml
[dependencies]
clingo = { version = "0.7.0-beta.1", features = ["derive", "dl-theory"] }
```

### Using `static-linking`

You can use the clingo library via static linking.

The recommended way to use the optional static linking support is as
follows.

```toml
[dependencies]
clingo = { version = "0.7.0-beta.1", features = ["static-linking"] }
```

*Attention: currently `static-linking` does not work with `dl-theory`.*

## Contribution

[How to make a contribution to `clingo-rs`?](https://github.com/potassco/clingo-rs/blob/master/CONTRIBUTING.md)

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
