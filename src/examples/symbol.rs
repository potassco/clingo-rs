extern crate clingo;

use clingo::*;


fn main() {
    let mut store = CStringStore::new();

    // create a number, identifier (function without arguments), and a function symbol
    let number_symbol = create_number(42);
    let identifier_symbol = store.create_id("x", true).unwrap();

    let mut symbols = vec![number_symbol, identifier_symbol];
    let function_symbol = store.create_function("x", &symbols, true).unwrap();
    symbols.push(function_symbol.clone());

    // print the symbols along with their hash values
    for ref symbol in &symbols {
        println!(
            "the hash of {} is {}",
            symbol.to_string().unwrap(),
            symbol.hash()
        );
    }

    // retrieve argument symbols of a symbol
    let symbols2 = function_symbol.arguments().unwrap();

    // equal to comparison
    for ref symbol in symbols2 {

        print!("{} is ", symbols[0].to_string().unwrap());
        if symbols[0] == *symbol {
            print!("equal");
        } else {
            print!("not equal");
        }
        println!(" to {}", symbol.to_string().unwrap());
    }

    // less than comparison
    print!("{} is ", symbols[0].to_string().unwrap());
    if symbols[0].is_less_than(&symbols[1]) {
        print!("less");
    } else {
        print!("not less");
    }
    println!(" than {}", symbols[1].to_string().unwrap());
}
