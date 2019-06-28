# clingo-rs

[![Build Status](https://travis-ci.org/potassco/clingo-rs.svg?branch=master)](https://travis-ci.org/potassco/clingo-rs)

Rust bindings to the [clingo](https://github.com/potassco/clingo) library.
Clingo version 5.3.0.

## Requirements

- a c++14 conforming compiler
  - *at least* [gcc](https://gcc.gnu.org/) version 4.9
  - [clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
    provided by gcc 4.9 or libc++)

## Compile & Test
    cargo build
    cargo test
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


## --dynamic_linking

The crate defines a [Cargo feature] that allows to use the clingo library via dynamic linking.

[Cargo feature]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

With dynamic linking enabled the clingo library is not build for static linking but it is assumed that a
clingo dynamic library is installed on the system.

The recommended way to use the optional dynamic linking support is as
follows.

```toml
[dependencies]
clingo = { version = "0.4.1", features = ["dynamic_linking"] }
```

## Contribution

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
