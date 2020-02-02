# clingo-rs
[![Build Status](https://travis-ci.org/potassco/clingo-rs.svg?branch=master)](https://travis-ci.org/potassco/clingo-rs)
[![Latest Version](https://img.shields.io/crates/v/clingo.svg)](https://crates.io/crates/clingo)
[![Rust Documentation](https://docs.rs/clingo/badge.svg)](https://docs.rs/clingo)

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

## Using `derive` macro

The crate provides a derive macro to help easing the use of rust data types as facts.


In your `Cargo.toml` add:

```toml
[dependencies]
clingo = { version = "0.7", features = ["derive"] }
```

In your source write:

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

The macro performs a conversion to snake case. This means the corresponing fact for `MyPoint{x:4,y:2}` is `my_point(4,2)`.

## Parallel build

It is possible to speed up the build of clingo, via parallel compilation. This is enabled by the feature `parallel_build`.

```toml
[dependencies]
clingo = { version = "0.6.0", features = ["parallel_build"] }
```

## Using `dynamic_linking`

The crate defines a [Cargo feature] that allows to use the clingo library via dynamic linking.

[Cargo feature]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

With dynamic linking enabled the clingo library is not build for static linking but it is assumed that a
clingo dynamic library is installed on the system.

The recommended way to use the optional dynamic linking support is as
follows.

```toml
[dependencies]
clingo = { version = "0.7.0", features = ["derive", "dynamic_linking"] }
```

## Contribution

[How to make a contribution to `clingo-rs`?](https://github.com/potassco/clingo-rs/blob/master/CONTRIBUTING.md)

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
