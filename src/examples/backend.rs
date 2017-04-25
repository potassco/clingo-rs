use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;


extern "C" fn on_model(model: &mut ClingoModel, data: *mut c_void, goon: *mut u8) -> u8 {

    // retrieve the symbols in the model
    let atoms = model.symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

    print!("Model:");
    for atom in atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        print!(" {}", atom_string.to_str().unwrap());
    }
    println!("");
    return 1;
}

fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn main() {

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, "{a; b; c}.");
    if err1 == 0 {
        return error_main();
    }

    // ground the base part
    let part = ClingoPart {
        name: CString::new("base").unwrap(),      
        params: &[],
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err2 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err2 == 0 {
        return error_main();
    }

    let atom_strings = ["a", "b", "c"];
    // get the ids of atoms a, b, and c
    let mut atom_ids = Vec::new();
    {
        // get symbolic atoms
        let atoms = ctl.symbolic_atoms().unwrap();

        for atom in atom_strings.iter() {
            let symbol = safe_clingo_symbol_create_id(atom, true).unwrap();
            let atom_it = atoms.find(symbol).unwrap();

            // get the atom's id
            let lit = atoms.literal(atom_it).unwrap();
            atom_ids.push(lit);
        }
    }

    {
        // get the backend
        let backend = ctl.backend().unwrap();

        // add an additional atom (called d below)
        let atom_d = backend.add_atom().unwrap();
        
        // add rule: d :- a, b.
        let head = vec![atom_d];
        let body = vec![atom_ids[0], atom_ids[1]];
        let err = backend.rule(false, &head, &body);
        if err == 0 {
            return error_main();
        }

        // add rule: :- not d, c.
        let head = vec![];
        let body = vec![-(atom_d as clingo_literal_t), atom_ids[2]];

        let err = backend.rule(false, &head, &body);
        if err == 0 {
            return error_main();
        }
    }
    
    // solve using a model callback
    let solve_callback: ClingoModelCallback = Some(on_model);
    let solve_callback_data = std::ptr::null_mut();
    let assumptions = vec![];
    let _solve_result = ctl.solve(solve_callback, solve_callback_data, assumptions)
        .expect("Failed while solving");

}
