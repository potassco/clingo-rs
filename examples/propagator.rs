use clingo::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn print_model(model: &Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for symbol in atoms {
        print!(" {}", symbol);
    }
    println!();
}

fn solve(ctl: GenericControl<CtrlCtx>) {
    // get a solve handle
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed to retrieve solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(Some(model)) => print_model(model),
            // stop if there are no more models
            Ok(None) => break,
            Err(e) => panic!("Error: {}", e),
        }
    }

    // close the solve handle
    handle.close().expect("Failed to close solve handle.");
}

// returns the offset'th numeric argument of the function symbol sym
fn get_arg(sym: &Symbol, offset: usize) -> Result<i32, ClingoError> {
    // get the arguments of the function symbol
    let args = sym.arguments().unwrap();
    // get the requested numeric argument
    args[offset as usize].number()
}

// state information for individual solving threads
#[derive(Debug)]
struct StateT {
    // assignment of pigeons to holes
    // (hole number -> pigeon placement literal or zero)
    holes: Vec<Option<SolverLiteral>>,
}

// state information for the propagator
struct PropagatorT {
    // mapping from solver literals capturing pigeon placements to hole numbers
    // (solver literal -> hole number or zero)
    pigeons: Vec<i32>,
    // array of states
    states: Vec<Rc<RefCell<StateT>>>,
}
impl Propagator for PropagatorT {
    fn init(&mut self, init: &mut PropagateInit) -> bool {
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

        if !self.states.is_empty() {
            // in principle the number of threads can increase between solve calls by changing the
            // configuration this case is not handled (elegantly) here
            if threads > self.states.len() {
                set_error(ErrorType::Runtime, "more threads than states").unwrap();
            }
            return true;
        }

        let s1_holes: Vec<Option<SolverLiteral>> = vec![];
        let state1 = Rc::new(RefCell::new(StateT { holes: s1_holes }));
        self.states = vec![state1];

        // create place/2 signature to filter symbolic atoms with
        let sig = Signature::new("place", 2, true).unwrap();

        // loop over the place/2 atoms in two passes
        // the first pass determines the maximum placement literal
        // the second pass allocates memory for data structures based on the first pass
        for pass in 0..2 {
            let mut watches = vec![];
            {
                // the propagator monitors place/2 atoms and dectects conflicting assignments
                // first get the symbolic atoms handle
                let atoms = init.symbolic_atoms().unwrap();

                // get an iterator for place/2 atoms
                // (atom order corresponds to grounding order (and is unpredictable))
                let mut atoms_iterator = atoms.iter_with_signature(sig).unwrap();

                if pass == 1 {
                    // allocate memory for the assignment literal -> hole mapping
                    self.pigeons = vec![0; max + 1];
                }

                while let Some(item) = atoms_iterator.next() {
                    // get the solver literal for the placement atom
                    let lit = init.solver_literal(item.literal().unwrap()).unwrap();
                    let lit_id = lit.get_integer() as usize;

                    if pass == 0 {
                        // determine the maximum literal
                        if lit_id > max {
                            max = lit_id;
                        }
                    } else {
                        // extract the hole number from the atom
                        let sym = item.symbol().unwrap();
                        let h = get_arg(&sym, 1).unwrap();

                        // initialize the assignment literal -> hole mapping
                        self.pigeons[lit_id] = h;

                        // watch the assignment literal
                        watches.push(lit);

                        // update the total number of holes
                        if h + 1 > holes {
                            holes = h + 1;
                        }
                    }
                }
            }
            // watch the assignment literals
            for lit in watches {
                init.add_watch(lit).expect("Failed to add watch.");
            }
        }

        // initialize the per solver thread state information
        for i in 0..threads {
            // initially no pigeons are assigned to any holes
            // so the hole -> literal mapping is initialized with zero
            // which is not a valid literal
            self.states[i].borrow_mut().holes = vec![None; holes as usize];
        }
        true
    }

    fn propagate(&mut self, control: &mut PropagateControl, changes: &[SolverLiteral]) -> bool {
        // get the thread specific state
        let mut state = self.states[control.thread_id() as usize].borrow_mut();

        // apply and check the pigeon assignments done by the solver
        for &lit in changes.iter() {
            // a pointer to the previously assigned literal
            let idx = self.pigeons[lit.get_integer() as usize] as usize;
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
                    let clause: &[SolverLiteral] = &[lit.negate(), x.negate()];
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
                    assert!(false);
                }
            };
        }
        true
    }

    fn undo(&mut self, control: &mut PropagateControl, changes: &[SolverLiteral]) {
        // get the thread specific state
        let mut state = self.states[control.thread_id() as usize].borrow_mut();

        // undo the assignments made in propagate
        for &lit in changes.iter() {
            let hole = self.pigeons[lit.get_integer() as usize] as usize;

            if let Some(x) = state.holes[hole] {
                if x == lit {
                    // undo the assignment
                    state.holes[hole] = None;
                }
            }
        }
    }
}

struct CtrlCtx {
    non: defaults::Non,
    propagator: PropagatorT,
}
impl ControlCtx for CtrlCtx {
    type L = defaults::Non;
    type P = PropagatorT;
    type O = defaults::Non;
    type F = defaults::Non;

    fn logger(&mut self) -> (&mut Self::L, u32) {
        (&mut self.non, 0)
    }
    fn propagator(&mut self) -> (&mut Self::P, bool) {
        (&mut self.propagator, false)
    }
    fn observer(&mut self) -> (&mut Self::O, bool) {
        (&mut self.non, false)
    }
    fn function_handler(&mut self) -> &mut Self::F {
        &mut self.non
    }
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a propagator with the functions above
    // using the default implementation for the model check
    let propagator = PropagatorT {
        pigeons: vec![],
        states: vec![],
    };

    // create a control context with the propagator
    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        propagator,
    };
    // create a control object and pass command line arguments
    match control_with_context(options, ctrl_ctx) {
        Ok(mut ctl) => {
            // add a logic program to the pigeon part
            // parameters for the pigeon part
            ctl.add(
                "pigeon",
                &vec!["h", "p"],
                "1 { place(P,H) : H = 1..h } 1 :- P = 1..p.",
            )
            .expect("Failed to add a logic program.");

            // ground the pigeon part

            // set the number of holes
            let arg0 = Symbol::create_number(3);
            // set the number of pigeons
            let arg1 = Symbol::create_number(2);

            let args = vec![arg0, arg1];

            // the pigeon program part having the number of holes and pigeons as parameters
            let part = Part::new("pigeon", args).unwrap();
            let parts = vec![part];
            ctl.ground(&parts)
                .expect("Failed to ground a logic program.");

            // solve using a model callback
            solve(ctl);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}
