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
//   size_t offset;
//   clingo_solve_result_bitset_t solve_ret;
//   clingo_control_t *ctl;
//   clingo_symbolic_atoms_t *atoms;
//   clingo_backend_t *backend;
//   clingo_atom_t atom_ids[4];
//   char const *atom_strings[] = {"a", "b", "c"};
//   clingo_literal_t body[2];
//   clingo_part_t parts[] = {{ "base", NULL, 0 }};

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = SafeClingoControl::new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, "{a; b; c}.");
    if err1 == 0 {
        return error_main();
    }
  
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
  
  // get the container for symbolic atoms
  if (!clingo_control_symbolic_atoms(ctl, &atoms)) { goto error; }
  // get the ids of atoms a, b, and c
  offset = 0;
  for (char const **it = atom_strings, **ie = it + sizeof(atom_strings) / sizeof(*atom_strings); it != ie; ++it) {
    clingo_symbol_t sym;
    clingo_symbolic_atom_iterator_t atom_it, atom_ie;
    clingo_literal_t lit;
    bool equal;

    // lookup the atom
    if (!clingo_symbol_create_id(*it, true, &sym)) { goto error; }
    if (!clingo_symbolic_atoms_find(atoms, sym, &atom_it)) { goto error; }
    if (!clingo_symbolic_atoms_end(atoms, &atom_ie)) { goto error; }
    if (!clingo_symbolic_atoms_iterator_is_equal_to(atoms, atom_it, atom_ie, &equal)) { goto error; }
    assert(!equal); (void)equal;

    // get the atom's id
    if (!clingo_symbolic_atoms_literal(atoms, atom_it, &lit)) { goto error; }
    atom_ids[offset++] = lit;
  }

  // get the backend
  if (!clingo_control_backend(ctl, &backend)) { goto error; }

  // add an additional atom (called d below)
  if (!clingo_backend_add_atom(backend, &atom_ids[3])) { goto error; }

  // add rule: d :- a, b.
  body[0] = atom_ids[0];
  body[1] = atom_ids[1];
  if (!clingo_backend_rule(backend, false, &atom_ids[3], 1, body, sizeof(body)/sizeof(*body))) { goto error; }

  // add rule: :- not d, c.
  body[0] = -(clingo_literal_t)atom_ids[3];
  body[1] = atom_ids[2];
  if (!clingo_backend_rule(backend, false, NULL, 0, body, sizeof(body)/sizeof(*body))) { goto error; }

  // solve using a model callback
  if (!clingo_control_solve(ctl, on_model, NULL, NULL, 0, &solve_ret)) { goto error; }

}

