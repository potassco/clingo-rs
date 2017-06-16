extern crate clingo;

use clingo::*;


fn main() {

    // create a number, identifier (function without arguments), and a function symbol
    let number_symbol = clingo_symbol::create_number(42);
    let identifier_symbol = clingo_symbol::create_id("x", true).unwrap();

    let function_args = &[number_symbol, identifier_symbol];
    let function_symbol = clingo_symbol::create_function("x", function_args, true).unwrap();
    let symbols = [number_symbol, identifier_symbol, function_symbol];

    // print the symbols along with their hash values
    for &symbol in &symbols {
        let atom_string = safe_clingo_symbol_to_string(&symbol).unwrap();
        println!(
            "the hash of {} is {}",
            atom_string.to_str().unwrap(),
            safe_clingo_symbol_hash(symbol)
        );
    }

    // retrieve argument symbols of a symbol
    let symbols2 = safe_clingo_symbol_arguments(function_symbol).unwrap();

    // equal to comparison
    for symbol in symbols2 {
        let equal = safe_clingo_symbol_is_equal_to(symbols[0], symbol);

        let atom_string1 = safe_clingo_symbol_to_string(&symbols[0]).unwrap();
        let atom_string2 = safe_clingo_symbol_to_string(&symbol).unwrap();
        if equal {
            println!(
                "{} is equal {}",
                atom_string1.to_str().unwrap(),
                atom_string2.to_str().unwrap()
            );
        } else {
            println!(
                "{} is not equal {}",
                atom_string1.to_str().unwrap(),
                atom_string2.to_str().unwrap()
            );
        }
    }

    // less than comparison
    let less = safe_clingo_symbol_is_less_than(symbols[0], symbols[1]);
    let atom_string1 = safe_clingo_symbol_to_string(&symbols[0]).unwrap();
    let atom_string2 = safe_clingo_symbol_to_string(&symbols[1]).unwrap();
    if less {
        println!(
            "{} is less than {}",
            atom_string1.to_str().unwrap(),
            atom_string2.to_str().unwrap()
        );
    } else {
        println!(
            "{} is not less than {}",
            atom_string1.to_str().unwrap(),
            atom_string2.to_str().unwrap()
        );
    }
}
