extern crate clingo;

fn main() {

    let (mo, mi, re) = clingo::safe_clingo_version();
    println!("Hello, this is clingo version {}.{}.{}.", mo, mi, re);
}
