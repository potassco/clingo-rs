
fn main() {
    println!("cargo:include=/home/sthiele/develop/clingo-rs/clingo/libgringo");
    println!("cargo:rustc-link-search=native=/home/sthiele/develop/clingo-rs/clingo/build/release");
    println!("cargo:rustc-link-lib=dylib=gringo");
    println!("cargo:rustc-link-lib=dylib=reify");
    println!("cargo:rustc-link-lib=dylib=lp");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=python3.5m");
}

