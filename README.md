# clingo-rs
Rust bindings to the [clingo](https://github.com/potassco/clingo) library.
Clingo version 5.2.2.

## Requirements

- a c++14 conforming compiler
  - *at least* [gcc](https://gcc.gnu.org/) version 4.9
  - [clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
    provided by gcc 4.9 or libc++)

## Compile & Test
    cargo +nightly build
    cargo +nightly test
    cargo +nightly run --example=ast 0
    cargo +nightly run --example=backend 0
    cargo +nightly run --example=configuration
    cargo +nightly run --example=control 0
    cargo +nightly run --example=model 0
    cargo +nightly run --example=propagator 0
    cargo +nightly run --example=solve-async 0
    cargo +nightly run --example=statistics 0
    cargo +nightly run --example=symbol 0
    cargo +nightly run --example=symbolic-atoms 0
    cargo +nightly run --example=theory-atoms 0
    cargo +nightly run --example=inject-terms 0
    cargo +nightly run --example=version

## Contribution

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
