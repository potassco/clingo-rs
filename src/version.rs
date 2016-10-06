extern crate clingo;

fn main() {

   let (mo,mi,re) = clingo::safe_clingo_version();
    println!("clingo version is: {}.{}.{}", mo, mi, re);
}
