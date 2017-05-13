extern crate clingo;

use std::env;
//extern crate libc;
//use libc::c_void;
//use libc::c_int;
use std::ffi::CString;
use clingo::*;


// state information for individual solving threads
struct state_t<'a> {
    // assignment of pigeons to holes
    // (hole number -> pigeon placement literal or zero)
    holes: &'a mut [clingo_literal_t],
    size: usize,
}

// state information for the propagator
struct propagator_t<'a> {
    // mapping from solver literals capturing pigeon placements to hole numbers
    // (solver literal -> hole number or zero)
    pigeons: *mut usize,
    pigeons_size: usize,
    // array of states
    states: &'a mut [state_t<'a>],
    states_size: usize,
}

fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

// returns the offset'th numeric argument of the function symbol sym
// fn get_arg(sym: clingo_symbol_t, offset: c_int, num: *mut c_int) -> bool {
//
//     // get the arguments of the function symbol
// //   if (!clingo_symbol_arguments(sym, &args, &args_size)) { return false; }
//     let args = safe_clingo_symbol_arguments(sym).unwrap();
//     // get the requested numeric argument
// //   if (!clingo_symbol_number(args[offset], num)) { return false; }
//     num = safe_clingo_symbol_number(args[offset as usize]).unwrap();
//
//     return true;
// }

// unsafe extern "C" fn init(init: *mut clingo_propagate_init_t, data: *mut c_void) -> u8 {
//     let mut propagato = data as *mut propagator_t;
//     let mut propagator = *propagato;
//     // the total number of holes pigeons can be assigned too
//     let holes: c_int = 0;
//     let threads: size_t = (*init).number_of_threads();
//     // stores the (numeric) maximum of the solver literals capturing pigeon placements
//     // note that the code below assumes that this literal is not negative
//     // which holds for the pigeon problem but not in general
//     let max: clingo_literal_t = 0;
//     let atoms: *mut clingo_symbolic_atoms_t;
//     let sig: clingo_signature_t;
//     let atoms_it: clingo_symbolic_atom_iterator_t;
//     let atoms_ie: clingo_symbolic_atom_iterator_t;
//     // ensure that solve can be called multiple times
//     // for simplicity, the case that additional holes or pigeons to assign are grounded is not handled here
//     //     if propagator.states != NULL {
//     if propagator.states_size != 0 {
//         // in principle the number of threads can increase between solve calls by changing the configuration
//         // this case is not handled (elegantly) here
//         if threads > propagator.states_size {
//             //       clingo_set_error(clingo_error_runtime, "more threads than states");
//             safe_clingo_set_error(1, "more threads than states");
//         }
//         return 1;
//     }
// allocate memory for exactly one state per thread
//   if (!(data->states = (state_t*)malloc(sizeof(*data->states) * threads))) {
// //     clingo_set_error(clingo_error_bad_alloc, "allocation failed");
//     safe_clingo_set_error(3, "allocation failed");
//     return false;
//   }

//   memset(data->states, 0, sizeof(*data->states) * threads);
//   data->states_size = threads;
//
//   // the propagator monitors place/2 atoms and dectects conflicting assignments
//   // first get the symbolic atoms handle
//   if (!clingo_propagate_init_symbolic_atoms(init, &atoms)) { return false; }
//   // create place/2 signature to filter symbolic atoms with
//   if (!clingo_signature_create("place", 2, true, &sig)) { return false; }
//   // get an iterator after the last place/2 atom
//   // (atom order corresponds to grounding order (and is unpredictable))
//   if (!clingo_symbolic_atoms_end(atoms, &atoms_ie)) { return false; }
//
//   // loop over the place/2 atoms in two passes
//   // the first pass determines the maximum placement literal
//   // the second pass allocates memory for data structures based on the first pass
//   for (int pass = 0; pass < 2; ++pass) {
//     // get an iterator to the first place/2 atom
//     if (!clingo_symbolic_atoms_begin(atoms, &sig, &atoms_it)) { return false; }
//     if (pass == 1) {
//       // allocate memory for the assignemnt literal -> hole mapping
//       if (!(data->pigeons = (int*)malloc(sizeof(*data->pigeons) * (max + 1)))) {
//         clingo_set_error(clingo_error_bad_alloc, "allocation failed");
//         return false;
//       }
//       data->pigeons_size = max + 1;
//     }
//     while (true) {
//       int h;
//       bool equal;
//       clingo_literal_t lit;
//       clingo_symbol_t sym;
//
//       // stop iteration if the end is reached
//       if (!clingo_symbolic_atoms_iterator_is_equal_to(atoms, atoms_it, atoms_ie, &equal)) { return false; }
//       if (equal) { break; }
//
//       // get the solver literal for the placement atom
//       if (!clingo_symbolic_atoms_literal(atoms, atoms_it, &lit)) { return false; }
//       if (!clingo_propagate_init_solver_literal(init, lit, &lit)) { return false; }
//
//       if (pass == 0) {
//         // determine the maximum literal
//         assert(lit > 0);
//         if (lit > max) { max = lit; }
//       }
//       else {
//         // extract the hole number from the atom
//         if (!clingo_symbolic_atoms_symbol(atoms, atoms_it, &sym)) { return false; }
//         if (!get_arg(sym, 1, &h)) { return false; }
//
//         // initialize the assignemnt literal -> hole mapping
//         data->pigeons[lit] = h;
//
//         // watch the assignment literal
//         if (!clingo_propagate_init_add_watch(init, lit)) { return false; }
//
//         // update the total number of holes
//         if (h + 1 > holes)   { holes = h + 1; }
//       }
//
//       // advance to the next placement atom
//       if (!clingo_symbolic_atoms_next(atoms, atoms_it, &atoms_it)) { return false; }
//     }
//   }
//
//   // initialize the per solver thread state information
//   for (size_t i = 0; i < threads; ++i) {
//     if (!(data->states[i].holes = (clingo_literal_t*)malloc(sizeof(*data->states[i].holes) * holes))) {
//       clingo_set_error(clingo_error_bad_alloc, "allocation failed");
//       return false;
//     }
//     // initially no pigeons are assigned to any holes
//     // so the hole -> literal mapping is initialized with zero
//     // which is not a valid literal
//     memset(data->states[i].holes, 0, sizeof(*data->states[i].holes) * holes);
//     data->states[i].size = holes;
//   }

//     return 1;
// }

// unsafe extern "C" fn propagate(control: *mut clingo_propagate_control_t,
//                                changes: *const clingo_literal_t,
//                                size: size_t,
//                                data: *mut c_void)
//                                -> u8 {
//     let mut propagato = data as *mut propagator_t;
//     let mut propagator = *propagato;
//     // get the thread specific state
//     //   let state: state_t = data->states[clingo_propagate_control_thread_id(control)];
//     let state: state_t = propagator.states[(*control).thread_id() as usize];
//
//
//     // apply and check the pigeon assignments done by the solver
//     for i in 0..size {
//         // the freshly assigned literal
//         let lit: clingo_literal_t = changes[i];
//         // a pointer to the previously assigned literal
//         let prev: *mut clingo_literal_t = state.holes + propagator.pigeons[lit];
//
//         // update the placement if no literal was assigned previously
//         if *prev == 0 {
//             *prev = lit;
//         }
//         // create a conflicting clause and propagate it
//         else {
//             // current and previous literal must not hold together
//             let clause: &[clingo_literal_t] = &[-lit, -*prev];
//             // stores the result when adding a clause or propagationg
//             // if result is false propagation must stop for the solver to backtrack
//
//             // add the clause
//             //       if (!clingo_propagate_control_add_clause(control, clause, sizeof(clause)/sizeof(*clause), clingo_clause_type_learnt, &result)) {return false; }
//             let result = (*control)
//                 .add_clause(clause,
//                             clingo_clause_type::clingo_clause_type_learnt as clingo_clause_type_t)
//                 .unwrap();
//
//             if result == 0 {
//                 return 1;
//             }
//
//             // propagate it
//             //       if (!clingo_propagate_control_propagate(control, &result)) { return false; }
//             result = (*control).propagate().unwrap();
//
//             if result == 0 {
//                 return 1;
//             }
//
//             // must not happen because the clause above is conflicting by construction
//             assert!(false);
//         }
//     }
//     return 1;
// }
//
// unsafe extern "C" fn undo(control: *mut clingo_propagate_control_t,
//                           changes: *const clingo_literal_t,
//                           size: size_t,
//                           data: *mut c_void)
//                           -> u8 {
//     let mut propagato = data as *mut propagator_t;
//     let mut propagator = *propagato;
//     // get the thread specific state
//     //   let state: state_t = data->states[clingo_propagate_control_thread_id(control)];
//     let state: state_t = propagator.states[(*control).thread_id() as usize];
//
//     // undo the assignments made in propagate
//     for i in 0..size {
//         let lit: clingo_literal_t = changes[i];
//         let hole: c_int = propagator.pigeons[lit];
//
//         if state.holes[hole] == lit {
//             // undo the assignment
//             state.holes[hole] = 0;
//         }
//     }
//     return 1;
// }
//
// unsafe extern "C" fn on_model(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {
//
//     // retrieve the symbols in the model
//     let atoms = (*model)
//         .symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
//         .expect("Failed to retrieve symbols in the model");
//
//     println!("Model:");
//     for atom in atoms {
//         // retrieve and print the symbol's string
//         let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
//         println!(" {}", atom_string.to_str().unwrap());
//     }
//     return 1;
// }

fn main() {
    //   char const *error_message;
    //   int ret = 0;
    //   clingo_solve_result_bitset_t solve_ret;
    //   clingo_control_t *ctl = NULL;
    // arguments to the pigeon program part
    //   clingo_symbol_t args[2];
    // the pigeon program part having the number of holes and pigeons as parameters
    //     clingo_part_t parts[] = {{ "pigeon", args, sizeof(args)/sizeof(*args) }};
    let part = ClingoPart {
        name: CString::new("pigeon").unwrap(),
        params: &[], // args
    };
    let parts = vec![part];
    // parameters for the pigeon part
    let params = ["h", "p"];
    // create a propagator with the functions above
    // using the default implementation for the model check
    let mut prop = clingo_propagator {
        //         init: Some(init),
        init: None,
        //         propagate: Some(propagate),
        propagate: None,
        //         undo: Some(undo),
        undo: None,
        check: None,
    };

    // user data for the propagator
    let mut states = [];
    let mut prop_data = propagator_t {
        pigeons: std::ptr::null_mut(),
        pigeons_size: 0,
        states: &mut states,
        states_size: 0,
    };

    // set the number of holes
    let arg0 = safe_clingo_symbol_create_number(8);
    // set the number of pigeons
    let arg1 = safe_clingo_symbol_create_number(9);

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // register the propagator
    let prop_data_ptr = unsafe {
        std::mem::transmute::<&mut propagator_t, *mut ::std::os::raw::c_void>(&mut prop_data)
    };
    if !ctl.register_propagator(&prop, prop_data_ptr, false) {
        return error_main();
    }

    //
    //   // add a logic program to the pigeon part
    //   if (!clingo_control_add(ctl, "pigeon", params, sizeof(params)/sizeof(*params),
    //                           "1 { place(P,H) : H = 1..h } 1 :- P = 1..p.")) { goto error; }
    //
    //   // ground the pigeon part
    //   if (!clingo_control_ground(ctl, parts, 1, NULL, NULL)) { goto error; }
    //
    //   // solve using a model callback
    //   if (!clingo_control_solve(ctl, on_model, NULL, NULL, 0, &solve_ret)) { goto error; }
    //
    //   goto out;
    //
    // error:
    //   if (!(error_message = clingo_error_message())) { error_message = "error"; }
    //
    //   printf("%s\n", error_message);
    //   ret = clingo_error_code();
    //
    // out:
    //   // free the propagator state
    //   if (prop_data.pigeons) { free(prop_data.pigeons); }
    //   if (prop_data.states_size > 0) {
    //     for (size_t i = 0; i < prop_data.states_size; ++i) {
    //       if (prop_data.states[i].holes) {
    //         free(prop_data.states[i].holes);
    //       }
    //     }
    //     free(prop_data.states);
    //   }
    //
    //   if (ctl) { clingo_control_free(ctl); }
    //
    //   return ret;
}
