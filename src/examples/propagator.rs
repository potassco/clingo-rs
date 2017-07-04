extern crate libc;
extern crate clingo;

use std::env;
use std::vec::Vec;
use std::cell::RefCell;
use std::rc::Rc;
use clingo::*;


// state information for individual solving threads
#[derive(Debug)]
struct StateT {
    // assignment of pigeons to holes
    // (hole number -> pigeon placement literal or zero)
    holes: Vec<Option<ClingoLiteral>>,
}

// state information for the propagator
struct PropagatorT {
    // mapping from solver literals capturing pigeon placements to hole numbers
    // (solver literal -> hole number or zero)
    pigeons: Vec<i32>,
    // array of states
    states: Vec<Rc<RefCell<StateT>>>,
}

// returns the offset'th numeric argument of the function symbol sym
fn get_arg(sym: ClingoSymbol, offset: usize) -> Result<i32, &'static str> {
    // get the arguments of the function symbol
    let args = sym.arguments().unwrap();
    // get the requested numeric argument
    args[offset as usize].number()
}

extern "C" fn init(init_: *mut clingo_propagate_init_t, data: *mut ::std::os::raw::c_void) -> bool {

    println!("init!");

    let mut init = unsafe { (init_ as *mut ClingoPropagateInit).as_mut() }.unwrap();
    let mut propagator = unsafe { (data as *mut PropagatorT).as_mut() }.unwrap();

    // the total number of holes pigeons can be assigned too
    let mut holes = 0;
    let threads = init.number_of_threads();

    // ensure that solve can be called multiple times
    // for simplicity, the case that additional holes or pigeons to assign are grounded is not handled here

    if propagator.states.len() != 0 {
        // in principle the number of threads can increase between solve calls by changing the configuration
        // this case is not handled (elegantly) here
        if threads > propagator.states.len() {
            clingo::set_error(
                clingo_error::clingo_error_runtime,
                "more threads than states",
            );
        }
        return true;
    }

    let s1_holes: Vec<Option<ClingoLiteral>> = vec![];
    let state1 = Rc::new(RefCell::new(StateT { holes: s1_holes }));
    propagator.states = vec![state1];

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

        loop {
            // stop iteration if the end is reached
            let equal = atoms.iterator_is_equal_to(atoms_it, atoms_ie).unwrap();
            if equal {
                break;
            }

            // get the solver literal for the placement atom
            let mut lit = atoms.literal(atoms_it).unwrap();
            lit = init.solver_literal(lit).unwrap();

            if pass != 0 {
                // extract the hole number from the atom
                let sym = atoms.symbol(atoms_it).unwrap();
                let h = get_arg(sym, 1).unwrap();

                // initialize the assignemnt literal -> hole mapping
                propagator.pigeons[lit.get_integer() as usize] = h;

                // watch the assignment literal
                init.add_watch(lit).expect("Failed to add watch.");

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
    // for i in 0..threads {
    //     if (!((*propagator).states[i].holes = (clingo_literal_t*)malloc(sizeof(*data->states[i].holes) * holes))) {
    //         safe_clingo_set_error(clingo_error::clingo_error_bad_alloc as clingo_error_t, "allocation failed");
    //         return false;
    //     }
    // initially no pigeons are assigned to any holes
    // so the hole -> literal mapping is initialized with zero
    // which is not a valid literal
    //     memset(data->states[i].holes, 0, sizeof(*data->states[i].holes) * holes);
    //     propagator.states[i as usize].borrow_mut().size) = holes as usize;
    // }
    return true;
}

extern "C" fn propagate(
    control_: *mut clingo_propagate_control_t,
    changes_: *const clingo_literal_t,
    size: usize,
    data: *mut ::std::os::raw::c_void,
) -> bool {

    println!("propagate!");

    let mut control = unsafe { (control_ as *mut ClingoPropagateControl).as_mut() }.unwrap();
    let changes = unsafe { std::slice::from_raw_parts(changes_ as *const ClingoLiteral, size) };
    let propagator = unsafe { (data as *mut PropagatorT).as_ref() }.unwrap();

    // get the thread specific state
    let mut state = propagator.states[control.thread_id() as usize].borrow_mut();

    // apply and check the pigeon assignments done by the solver
    for i in 0..size {
        // the freshly assigned literal
        let lit: ClingoLiteral = changes[i];
        // a pointer to the previously assigned literal
        let idx = propagator.pigeons[lit.get_integer() as usize] as usize;
        let mut prev = state.holes[idx];

        // update the placement if no literal was assigned previously
        match prev {
            None => {
                prev = Some(lit);
                state.holes[idx] = prev;
            }
            // create a conflicting clause and propagate it
            Some(x) => {
                // current and previous literal must not hold together
                let clause: &[ClingoLiteral] = &[lit.negate(), x.negate()];
                // stores the result when adding a clause or propagationg
                // if result is false propagation must stop for the solver to backtrack

                // add the clause
                if !control
                    .add_clause(clause, clingo_clause_type_learnt)
                    .unwrap()
                {
                    return true;
                }

                // propagate it
                if !control.propagate().unwrap() {
                    return true;
                }

                // must not happen because the clause above is conflicting by construction
                // assert!(false);
            }
        };
    }
    return true;
}

extern "C" fn undo(
    control_: *mut clingo_propagate_control_t,
    changes_: *const clingo_literal_t,
    size: usize,
    data: *mut ::std::os::raw::c_void,
) -> bool {

    println!("undo!");

    let mut control = unsafe { (control_ as *mut ClingoPropagateControl).as_mut() }.unwrap();
    let changes = unsafe { std::slice::from_raw_parts(changes_ as *const ClingoLiteral, size) };
    let propagator = unsafe { (data as *mut PropagatorT).as_ref() }.unwrap();

    // get the thread specific state
    let mut state = propagator.states[control.thread_id() as usize].borrow_mut();

    // undo the assignments made in propagate
    for i in 0..size {
        let lit: ClingoLiteral = changes[i];
        let hole = propagator.pigeons[lit.get_integer() as usize] as usize;

        if let Some(x) = state.holes[hole] {
            if x == lit {
                // undo the assignment
                println!("TODO: holes{}:{:?}", hole, state.holes[hole]);
                state.holes[hole] = None;
            }
        }
    }
    return true;
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model.");

    print!(" Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!("");
}

fn solve(ctl: &mut ClingoControl) {

    let solve_mode = clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed to retrieve solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // stop if there are no more models
            Err(_) => break,
            // print the model
            Ok(model) => print_model(model),
        }
    }

    // close the solve handle
    let _result = handle.get();
    handle.close().expect("Failed to close solve handle.");
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a propagator with the functions above
    // using the default implementation for the model check
    let prop = ClingoPropagator::new(Some(init), Some(propagate), Some(undo), None);

    // user data for the propagator
    let mut prop_data = PropagatorT {
        pigeons: vec![],
        states: vec![],
    };

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let option = ClingoControl::new(options, logger, logger_data, 20);
    match option {
        Ok(ctl) => {
            // register the propagator
            let prop_data_ptr = unsafe {
                std::mem::transmute::<&mut PropagatorT, *mut ::std::os::raw::c_void>(&mut prop_data)
            };
            ctl.register_propagator(&prop, prop_data_ptr, false)
                .expect("Failed to register propagator.");

            // add a logic program to the pigeon part
            // parameters for the pigeon part
            let parameters = vec!["h", "p"];
            ctl.add(
                "pigeon",
                parameters,
                "1 { place(P,H) : H = 1..h } 1 :- P = 1..p.",
            ).expect("Failed to add a logic program.");

            print!("");

            // ground the pigeon part

            // set the number of holes
            let arg0 = ClingoSymbol::create_number(3);
            // set the number of pigeons
            let arg1 = ClingoSymbol::create_number(3);

            let mut args = Vec::new();
            args.push(arg0);
            args.push(arg1);

            // the pigeon program part having the number of holes and pigeons as parameters

            let part = ClingoPart::new_part("pigeon", args.as_slice());
            let parts = vec![part];
            let ground_callback = None;
            let ground_callback_data = std::ptr::null_mut();
            ctl.ground(parts, ground_callback, ground_callback_data)
                .expect("Failed to ground a logic program.");

            // solve using a model callback
            solve(ctl);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
