extern crate clingo;

use std::env;
use std::vec::Vec;
use std::cell::RefCell;
use std::rc::Rc;
use clingo::*;

fn print_model(model: &mut Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(&ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!();
}

fn solve(ctl: &mut Control) {
    // get a solve handle
    let handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed to retrieve solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(model) => print_model(model),
            // stop if there are no more models
            Err(_) => break,
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
}

// state information for individual solving threads
#[derive(Debug)]
struct StateT {
    // assignment of pigeons to holes
    // (hole number -> pigeon placement literal or zero)
    holes: Vec<Option<Literal>>,
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
fn get_arg(sym: &Symbol, offset: usize) -> Result<i32, Error> {
    // get the arguments of the function symbol
    let args = sym.arguments().unwrap();
    // get the requested numeric argument
    args[offset as usize].number()
}

struct MyPropagator;
impl Propagator<PropagatorT> for MyPropagator {
    fn init(init: &mut PropagateInit, propagator: &mut PropagatorT) -> bool {
        // stores the (numeric) maximum of the solver literals capturing pigeon placements
        // note that the code below assumes that this literal is not negative
        // which holds for the pigeon problem but not in general
        let mut max = 0;

        // the total number of holes pigeons can be assigned too
        let mut holes = 0;
        let threads = init.number_of_threads();

        // ensure that solve can be called multiple times
        // for simplicity, the case that additional holes or pigeons to assign are grounded is not
        // handled here

        if !propagator.states.is_empty() {
            // in principle the number of threads can increase between solve calls by changing the
            // configuration this case is not handled (elegantly) here
            println!("hi propagator.states.is_not_empty");
            if threads > propagator.states.len() {
                set_error(ErrorType::Runtime, "more threads than states");
            }
            return true;
        }

        let s1_holes: Vec<Option<Literal>> = vec![];
        let state1 = Rc::new(RefCell::new(StateT { holes: s1_holes }));
        propagator.states = vec![state1];

        // the propagator monitors place/2 atoms and dectects conflicting assignments
        // first get the symbolic atoms handle
        let atoms = init.symbolic_atoms().unwrap();

        // create place/2 signature to filter symbolic atoms with
        let sig = Signature::create("place", 2, true).unwrap();

        // get an iterator after the last place/2 atom
        // (atom order corresponds to grounding order (and is unpredictable))
        let atoms_ie = atoms.end().unwrap();

        // loop over the place/2 atoms in two passes
        // the first pass determines the maximum placement literal
        // the second pass allocates memory for data structures based on the first pass
        for pass in 0..2 {
            // get an iterator to the first place/2 atom
            let mut atoms_it = atoms.begin(Some(&sig)).unwrap();

            if pass == 1 {
                // allocate memory for the assignemnt literal -> hole mapping
                propagator.pigeons = vec![0; max + 1];;
            }

            loop {
                // stop iteration if the end is reached
                let equal = atoms.iterator_is_equal_to(atoms_it, atoms_ie).unwrap();
                if equal {
                    break;
                }

                // get the solver literal for the placement atom
                let lit = init.solver_literal(atoms.literal(atoms_it).unwrap())
                    .unwrap();
                let lit_id = lit.get_integer() as usize;

                if pass == 0 {
                    // determine the maximum literal
                    if lit_id > max {
                        max = lit_id;
                    }
                } else {
                    // extract the hole number from the atom
                    let sym = atoms.symbol(atoms_it).unwrap();
                    let h = get_arg(&sym, 1).unwrap();

                    // initialize the assignemnt literal -> hole mapping
                    propagator.pigeons[lit_id] = h;

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
        for i in 0..threads {
            // initially no pigeons are assigned to any holes
            // so the hole -> literal mapping is initialized with zero
            // which is not a valid literal
            propagator.states[i].borrow_mut().holes = vec![None; holes as usize];
        }
        true
    }

    fn propagate(
        control: &mut PropagateControl,
        changes: &[Literal],
        propagator: &mut PropagatorT,
    ) -> bool {
        // get the thread specific state
        let mut state = propagator.states[control.thread_id() as usize].borrow_mut();

        // apply and check the pigeon assignments done by the solver
        for &lit in changes.iter() {
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
                    let clause: &[Literal] = &[lit.negate(), x.negate()];
                    // stores the result when adding a clause or propagationg
                    // if result is false propagation must stop for the solver to backtrack

                    // add the clause
                    if !control.add_clause(clause, ClauseType::Learnt).unwrap() {
                        return true;
                    }

                    // propagate it
                    if !control.propagate().unwrap() {
                        return true;
                    }

                    // must not happen because the clause above is conflicting by construction
                    println!("assert!(false) line 226 propagator.rs");
                    assert!(false);
                }
            };
        }
        true
    }

    fn undo(
        control: &mut PropagateControl,
        changes: &[Literal],
        propagator: &mut PropagatorT,
    ) -> bool {
        // get the thread specific state
        let mut state = propagator.states[control.thread_id() as usize].borrow_mut();

        // undo the assignments made in propagate
        for &lit in changes.iter() {
            let hole = propagator.pigeons[lit.get_integer() as usize] as usize;

            if let Some(x) = state.holes[hole] {
                if x == lit {
                    // undo the assignment
                    state.holes[hole] = None;
                }
            }
        }
        true
    }
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a propagator with the functions above
    // using the default implementation for the model check
    let prop = MyPropagator;

    // user data for the propagator
    let mut prop_data = PropagatorT {
        pigeons: vec![],
        states: vec![],
    };

    // create a control object and pass command line arguments
    let option = Control::new(options);

    match option {
        Ok(mut ctl) => {
            // register the propagator
            ctl.register_propagator(&prop, &mut prop_data, false)
                .expect("Failed to register propagator.");

            // add a logic program to the pigeon part
            // parameters for the pigeon part
            let parameters = vec!["h", "p"];
            ctl.add(
                "pigeon",
                parameters,
                "1 { place(P,H) : H = 1..h } 1 :- P = 1..p.",
            ).expect("Failed to add a logic program.");

            // ground the pigeon part

            // set the number of holes
            let arg0 = Symbol::create_number(8);
            // set the number of pigeons
            let arg1 = Symbol::create_number(8);

            let mut args = Vec::new();
            args.push(arg0);
            args.push(arg1);

            // the pigeon program part having the number of holes and pigeons as parameters
            let part = Part::new("pigeon", args.as_slice());
            let parts = vec![part];
            ctl.ground(&parts)
                .expect("Failed to ground a logic program.");

            // solve using a model callback
            solve(&mut ctl);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
