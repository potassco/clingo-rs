# clingo-rs [![Build Status](https://github.com/potassco/clingo-rs/actions/workflows/test-ci.yml/badge.svg)](https://github.com/potassco/clingo-rs/actions/workflows/test-ci.yml) [![Latest Version](https://img.shields.io/crates/v/clingo.svg)](https://crates.io/crates/clingo) [![Rust Documentation](https://docs.rs/clingo/badge.svg)](https://docs.rs/clingo)

Rust idiomatic bindings to the [clingo](https://github.com/potassco/clingo) library.
Clingo version 5.6.2.

## Usage

### Default - dynamic linking

Per default the crate uses the clingo library via dynamic linking.
It is assumed that a clingo dynamic library is installed on the system.
While compile time the environment variable `CLINGO_LIBRARY_PATH` must be set. For example:

```sh
export CLINGO_LIBRARY_PATH=/scratch/miniconda3/envs/test/lib
```

And for running the code that dynamically links to clingo make sure to have the clingo library in your `LD_LIBRARY_PATH`.

Have a look a at the [clingo Readme](https://github.com/potassco/clingo#readme) for different ways of obtaining the binary releases of clingo.

### Using `static-linking`

You can also use the clingo library via static linking.
The build system will attempt to compile clingo for static linking on your system.
To build clingo for static linking you need the following tools installed:

- a C++14 conforming compiler
  - *at least* [GCC](https://gcc.gnu.org/) version 4.9
  - [Clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
    provided by gcc 4.9 or libc++)
  - *at least* MSVC 15.0 ([Visual Studio](https://www.visualstudio.com/) 2017)
  - other compilers might work
- the [cmake](https://www.cmake.org/) build system
  - at least version 3.18 is recommended
  - at least version 3.1 is *required*

The recommended way to use the optional static linking support is as
follows.

```toml
[dependencies]
clingo = { version = "0.8.0", features = ["static-linking"] }
```

### Using `derive` macro

The crate provides a derive macro to help ease the use of rust data types as facts.

In your `Cargo.toml` add:

```toml
[dependencies]
clingo = { version = "0.8.0", features = ["derive"] }
```

In your source write:

```ignore
use clingo::{ClingoError, FactBase, Symbol, ToSymbol};

#[derive(ToSymbol)]
struct MyPoint {
    x: i32,
    y: i32,
}

let p = MyPoint { x: 4, y: 2 };
let mut fb = FactBase::new();
fb.insert(&p);
```

The macro performs a conversion to snake case. This means the corresponding fact for `MyPoint{x:4,y:2}` is `my_point(4,2)`.

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

## Contribution

[How to make a contribution to `clingo-rs`?](https://github.com/potassco/clingo-rs/blob/master/CONTRIBUTING.md)

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
