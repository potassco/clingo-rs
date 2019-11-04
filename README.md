# clingo-rs

[![Build Status](https://travis-ci.org/potassco/clingo-rs.svg?branch=master)](https://travis-ci.org/potassco/clingo-rs)

Rust bindings to the [clingo](https://github.com/potassco/clingo) library.
Clingo version 5.4.0.

## Requirements

- a c++14 conforming compiler
  - *at least* [gcc](https://gcc.gnu.org/) version 4.9
  - [clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
    provided by gcc 4.9 or libc++)

## Examples

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


## Documentation

- [`clingo-rs`](https://docs.rs/clingo)

## `clingo_derive` crate

The [`clingo_derive`](https://crates.io/crates/clingo-derive) crate helps easing the use of rust data types as facts.

In your `Cargo.toml` add:

    [dependencies]
    clingo-rs = "0.5.0"
    clingo-derive = "*"

In your source write:

    use clingo_derive::*;
    use clingo::FactBase;

    #[derive(ToSymbol)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point{ x:4, y:2 };
    let fb = FactBase::new();
    fb.insert(p);


## --dynamic_linking

The crate defines a [Cargo feature] that allows to use the clingo library via dynamic linking.

[Cargo feature]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

With dynamic linking enabled the clingo library is not build for static linking but it is assumed that a
clingo dynamic library is installed on the system.

The recommended way to use the optional dynamic linking support is as
follows.

```toml
[dependencies]
clingo = { version = "0.5.0", features = ["dynamic_linking"] }
```

## Contribution

[How to make a contribution to `clingo-rs`?](https://github.com/potassco/clingo-rs/blob/master/CONTRIBUTING.md)

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
