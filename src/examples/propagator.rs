extern crate libc;
extern crate clingo;

use std::env;
use std::ffi::CString;
use std::vec::Vec;
use std::cell::RefCell;
use std::rc::Rc;
use libc::c_int;
use clingo::*;


// state information for individual solving threads
#[derive(Debug)]
struct StateT {
    // assignment of pigeons to holes
    // (hole number -> pigeon placement literal or zero)
    holes: Vec<clingo_literal_t>,
    size: usize,
}

// state information for the propagator
struct PropagatorT {
    // mapping from solver literals capturing pigeon placements to hole numbers
    // (solver literal -> hole number or zero)
    pigeons: Vec<i32>,
    pigeons_size: i32,
    // array of states
    states: Vec<Rc<RefCell<StateT>>>,
    states_size: i32,
}

fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("Error: {}", error_message);
    safe_clingo_error_code();
}

// returns the offset'th numeric argument of the function symbol sym
fn get_arg(sym: clingo_symbol_t, offset: c_int) -> Option<c_int> {
    // get the arguments of the function symbol
    let args = safe_clingo_symbol_arguments(sym).unwrap();
    // get the requested numeric argument
    safe_clingo_symbol_number(args[offset as usize])
}

extern "C" fn init(init_: *mut clingo_propagate_init_t, data: *mut ::std::os::raw::c_void) -> bool {
    let mut init = unsafe {
            std::mem::transmute::<*mut clingo_propagate_init_t, *mut ClingoPropagateInit>(init_)
                .as_mut()
        }
        .unwrap();
    let mut propagator = unsafe { (data as *mut PropagatorT).as_mut() }.unwrap();

    // the total number of holes pigeons can be assigned too
    let mut holes = 0;
    let threads = init.number_of_threads();
    // stores the (numeric) maximum of the solver literals capturing pigeon placements
    // note that the code below assumes that this literal is not negative
    // which holds for the pigeon problem but not in general
    let mut max: clingo_literal_t = 0;
    // ensure that solve can be called multiple times
    // for simplicity, the case that additional holes or pigeons to assign are grounded is not handled here

    if propagator.states_size != 0 {
        // in principle the number of threads can increase between solve calls by changing the configuration
        // this case is not handled (elegantly) here
        if threads > propagator.states_size {
            safe_clingo_set_error(clingo_error::clingo_error_runtime as clingo_error_t,
                                  "more threads than states");
        }
        return true;
    }
    // allocate memory for exactly one state per thread
    // if (!(data->states = (StateT*)malloc(sizeof(*data->states) * threads))) {
    //     safe_clingo_set_error(clingo_error::clingo_error_bad_alloc as clingo_error_t, "allocation failed");
    //     return false;
    // }
    //   memset(data->states, 0, sizeof(*data->states) * threads);
    let s1_holes: Vec<clingo_literal_t> = vec![];
    let state1 = Rc::new(RefCell::new(StateT {
                                          holes: s1_holes,
                                          size: 0,
                                      }));
    propagator.states = vec![state1];
    propagator.states_size = threads;

    // the propagator monitors place/2 atoms and dectects conflicting assignments
    // first get the symbolic atoms handle
    let atoms = init.symbolic_atoms().unwrap();

    // create place/2 signature to filter symbolic atoms with
    let sig = ClingoSignature::create("place", 2, true).unwrap();

    // get an iterator after the last place/2 atom
    // (atom order corresponds to grounding order (and is unpredictable))
    let atoms_ie = atoms.end().unwrap();

    // loop over the place/2 atoms in two passes
    // the first pass determines the maximum placement literal
    // the second pass allocates memory for data structures based on the first pass
    for pass in 0..1 {
        // get an iterator to the first place/2 atom
        let mut atoms_it = atoms.begin(Some(&sig)).unwrap();
        if pass == 1 {
            // allocate memory for the assignemnt literal -> hole mapping
            // if (!(data->pigeons = (int*)malloc(sizeof(*data->pigeons) * (max + 1)))) {
            //   safe_clingo_set_error(clingo_error::clingo_error_bad_alloc as clingo_error_t, "allocation failed");
            //   return false;
            // }
            propagator.pigeons_size = max + 1;
        }
        loop {
            // stop iteration if the end is reached
            let equal = atoms.iterator_is_equal_to(atoms_it, atoms_ie).unwrap();
            if equal {
                break;
            }

            // get the solver literal for the placement atom
            let mut lit = atoms.literal(atoms_it).unwrap();
            lit = init.solver_literal(lit).unwrap();

            if pass == 0 {
                // determine the maximum literal
                assert!(lit > 0, "lit not > 0");
                if lit > max {
                    max = lit;
                }
            } else {
                // extract the hole number from the atom
                let sym = atoms.symbol(atoms_it).unwrap();
                let h = get_arg(sym, 1).unwrap();

                // initialize the assignemnt literal -> hole mapping
                propagator.pigeons[lit as usize] = h;

                // watch the assignment literal
                if !init.add_watch(lit) {
                    return false;
                }

                // update the total number of holes
                if h + 1 > holes {
                    holes = h + 1;
                }
            }

            // advance to the next placement atom
            atoms_it = atoms.next(atoms_it).unwrap();
        }
    }

    // initialize the per solver thread state information
    for i in 0..threads {
        // if (!((*propagator).states[i].holes = (clingo_literal_t*)malloc(sizeof(*data->states[i].holes) * holes))) {
        //   safe_clingo_set_error(clingo_error::clingo_error_bad_alloc as clingo_error_t, "allocation failed");
        //   return false;
        // }
        // initially no pigeons are assigned to any holes
        // so the hole -> literal mapping is initialized with zero
        // which is not a valid literal
        // memset(data->states[i].holes, 0, sizeof(*data->states[i].holes) * holes);

        propagator.states[i as usize].borrow_mut().size = holes as usize;
    }
    return true;
}

extern "C" fn propagate(control_: *mut clingo_propagate_control_t,
                        changes_: *const clingo_literal_t,
                        size: usize,
                        data: *mut ::std::os::raw::c_void)
                        -> bool {
    let mut control = unsafe {
            std::mem::transmute::<*mut clingo_propagate_control_t,
                                  *mut ClingoPropagateControl>(control_)
                    .as_mut()
        }
        .unwrap();
    let changes = unsafe { std::slice::from_raw_parts(changes_, size) };
    let propagator = unsafe { (data as *mut PropagatorT).as_ref() }.unwrap();

    // get the thread specific state
    let mut state = propagator.states[control.thread_id() as usize].borrow_mut();

    // apply and check the pigeon assignments done by the solver
    for i in 0..size {
        // the freshly assigned literal
        let lit: clingo_literal_t = changes[i];
        // a pointer to the previously assigned literal
        let idx = propagator.pigeons[lit as usize] as usize;
        let mut prev = state.holes[idx];

        // update the placement if no literal was assigned previously
        if prev == 0 {
            prev = lit;
            state.holes[idx] = prev;
        }
        // create a conflicting clause and propagate it
        else {
            // current and previous literal must not hold together
            let clause: &[clingo_literal_t] = &[-lit, -prev];
            // stores the result when adding a clause or propagationg
            // if result is false propagation must stop for the solver to backtrack

            // add the clause
            if !control
                    .add_clause(clause, clingo_clause_type::clingo_clause_type_learnt)
                    .unwrap() {
                return true;
            }

            // propagate it
            if !control.propagate().unwrap() {
                return true;
            }

            // must not happen because the clause above is conflicting by construction
            //         assert!(false);
        }
    }
    return true;
}

extern "C" fn undo(control_: *mut clingo_propagate_control_t,
                   changes_: *const clingo_literal_t,
                   size: usize,
                   data: *mut ::std::os::raw::c_void)
                   -> bool {
    let mut control = unsafe {
            std::mem::transmute::<*mut clingo_propagate_control_t,
                                  *mut ClingoPropagateControl>(control_)
                    .as_mut()
        }
        .unwrap();
    let changes = unsafe { std::slice::from_raw_parts(changes_, size) };
    let propagator = unsafe { (data as *mut PropagatorT).as_ref() }.unwrap();

    // get the thread specific state
    let mut state = propagator.states[control.thread_id() as usize].borrow_mut();

    // undo the assignments made in propagate
    for i in 0..size {
        let lit: clingo_literal_t = changes[i];
        let hole: c_int = propagator.pigeons[lit as usize];

        if state.holes[hole as usize] == lit {
            // undo the assignment
            println!("TODO: holes{}:{}", hole, state.holes[hole as usize]);
            state.holes[hole as usize] = 0;
        }
    }
    return true;
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

    for atom in atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        print!(" {}", atom_string.to_str().unwrap());
    }
    println!("");
}

fn solve(ctl: &mut ClingoControl) {
    let solve_mode = clingo_solve_mode::clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed retrieving solve handle");

    // loop over all models
    loop {
        if !handle.resume() {
            return error_main();
        }
        match handle.model() {
            // stop if there are no more models
            None => break,
            // print the model
            Some(model) => print_model(model),
        }
    }

    // close the solve handle
    let _result = handle.get().expect("Failed to get solve handle");
    handle.close();
}

fn main() {
    // create a propagator with the functions above
    // using the default implementation for the model check
    let prop = clingo_propagator {
        init: Some(init),
        propagate: Some(propagate),
        undo: Some(undo),
        check: None,
    };

    // user data for the propagator
    let mut prop_data = PropagatorT {
        pigeons: vec![],
        pigeons_size: 0,
        states: vec![],
        states_size: 0,
    };

    // set the number of holes
    let arg0 = clingo_symbol::create_number(8);
    // set the number of pigeons
    let arg1 = clingo_symbol::create_number(9);

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let option = new_clingo_control(env::args(), logger, logger_data, 20);
    match option {
        Some(ctl) => {
            // register the propagator
            let prop_data_ptr =
                unsafe {
                    std::mem::transmute::<&mut PropagatorT,
                                          *mut ::std::os::raw::c_void>(&mut prop_data)
                };
            if !ctl.register_propagator(&prop, prop_data_ptr, false) {
                return error_main();
            }

            // add a logic program to the pigeon part
            // parameters for the pigeon part
            let parameters = vec!["h", "p"];
            let err = ctl.add("pigeon",
                              parameters,
                              "1 { place(P,H) : H = 1..h } 1 :- P = 1..p.");
            if !err {
                return error_main();
            }

            // ground the pigeon part
            // arguments to the pigeon program part
            let num_pigeons = u64::from_str_radix(env::args().nth(1).unwrap().as_str(), 10)
                .unwrap();
            // the pigeon program part having the number of holes and pigeons as parameters
            let part = ClingoPart {
                name: CString::new("pigeon").unwrap(),
                params: &[num_pigeons],
            };
            let parts = vec![part];
            let ground_callback = None;
            let ground_callback_data = std::ptr::null_mut();
            let err = ctl.ground(parts, ground_callback, ground_callback_data);
            if !err {
                return error_main();
            }

            // solve using a model callback
            solve(ctl);
        }
        None => {
            return error_main();
        }
    }
}
