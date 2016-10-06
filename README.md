# libclingo-rs
Rust bindings to clingo C API

# Dependencies
scons, gcc, re2c, bison.
For details see clingo/INSTALL Requirements
 
# Compile & Test
cd libclingo-rs/clingo
scons --duild-dir=release
cd ..
cargo build
cargo test
cargo run
