# libclingo-rs
Rust bindings to clingo C API

# Dependencies
Rust, scons, gcc, re2c, bison.
For details see [clingo/INSTALL](https://github.com/sthiele/clingo/blob/master/INSTALL) Requirements.
 
# Compile & Test
    cd libclingo-rs/clingo
    scons --build-dir=release
    cd ..
    cargo build
    cargo test
    cargo run
