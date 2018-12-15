fn main() {
    let (mo, mi, re) = clingo::version();
    println!("Hello, this is clingo version {}.{}.{}.", mo, mi, re);
}
