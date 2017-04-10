use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;


extern "C" fn on_model(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {

    // retrieve the symbols in the model
    let atoms = safe_clingo_model_symbols(model, clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

    println!("Model:");
    for atom in &atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        println!(" {}", atom_string.to_str().unwrap());
    }
    return 1;
}


fn main() {
//   char const *error_message;
//   int ret = 0;
//   size_t size;
//   clingo_solve_result_bitset_t solve_ret;
//   clingo_control_t *ctl = NULL;
//   clingo_part_t parts[] = {{ "base", NULL, 0 }};
//   clingo_theory_atoms_t *atoms;
//   clingo_literal_t lit = 0;


    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = SafeClingoControl::new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, 
                        "#theory t { \
                        term   { + : 1, binary, left };\
                        &a/0 : term, any;\
                        &b/1 : term, {=}, term, any\
                        .\
                        x :- &a { 1+2 }.\
                        y :- &b(3) { } = 17."
                      )

    // ground the base part
    let part = safe_clingo_part {
        params: 0,
        size: 0,
        name: CString::new("base").unwrap(),
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err2 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err2 == 0 {
        return error_main();
    }

//   // get the theory atoms container
//   if (!clingo_control_theory_atoms(ctl, &atoms)) { goto error; }
// 
//   // print the number of grounded theory atoms
//   if (!clingo_theory_atoms_size(atoms, &size)) { goto error; }
//   printf("number of grounded theory atoms: %zu\n", size);
// 
//   // verify that theory atom b has a guard
//   for (clingo_id_t atom = 0; atom < size; ++atom) {
//     clingo_id_t term;
//     char const *name;
// 
//     // get the term associated with the theory atom
//     if (!clingo_theory_atoms_atom_term(atoms, atom, &term)) { goto error; }
//     // get the name associated with the theory atom
//     if (!clingo_theory_atoms_term_name(atoms, term, &name)) { goto error; }
//     // we got theory atom b/1 here
//     if (strcmp(name, "b") == 0) {
//       bool guard;
//       if (!clingo_theory_atoms_atom_has_guard(atoms, atom, &guard)) { goto error; }
//       printf("theory atom b/1 has a guard: %s\n", guard ? "true" : "false");
//       // get the literal associated with the theory atom
//       if (!clingo_theory_atoms_atom_literal(atoms, atom, &lit)) { goto error; }
//       break;
//     }
//   }
// 
//   // use the backend to assume that the theory atom is true
//   // (note that only symbolic literals can be passed as assumptions to a solve call;
//   // the backend accepts any aspif literal)
//   if (lit != 0) {
//     clingo_backend_t *backend;
// 
//     // get the backend
//     if (!clingo_control_backend(ctl, &backend)) { goto error; }
//     // add the assumption
//     if (!clingo_backend_assume(backend, &lit, 1)) { goto error; }
//   }
// 

    // solve using a model callback
    let solve_callback: clingo_model_callback_t = Some(on_model);
    let solve_callback_data = std::ptr::null_mut();
    let assumptions = vec![];
    let solve_result = ctl.solve(solve_callback, solve_callback_data, assumptions)
                           .expect("Failed while solving");
}

