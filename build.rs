
fn main() {
    println!("cargo:include=clingo/libgringo");
    println!("cargo:rustc-link-search=native=clingo/build/release");
    println!("cargo:rustc-link-lib=dylib=gringo");
    println!("cargo:rustc-link-lib=dylib=reify");
    println!("cargo:rustc-link-lib=dylib=lp");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=python3.5m");
}

