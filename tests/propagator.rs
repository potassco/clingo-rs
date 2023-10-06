use clingo::*;
use std::cell::RefCell;
use std::rc::Rc;
use test_case::test_case;

#[derive(Debug)]
struct StateT {
    // assignment of pigeons to holes
    // (hole number -> pigeon placement literal or zero)
    holes: Vec<Option<SolverLiteral>>,
}

// returns the offset'th numeric argument of the function symbol sym
fn get_arg(sym: &Symbol, offset: usize) -> Result<i32, ClingoError> {
    // get the arguments of the function symbol
    let args = sym.arguments().unwrap();
    // get the requested numeric argument
    args[offset as usize].number()
}

struct CtrlCtx<P: Propagator> {
    non: defaults::Non,
    propagator: P,
}
impl<P: Propagator> ControlCtx for CtrlCtx<P> {
    type L = defaults::Non;
    type P = P;
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

struct PigeonPropagator {
    // mapping from solver literals capturing pigeon placements to hole numbers
    // (solver literal -> hole number or zero)
    pigeons: Vec<i32>,
    // array of states
    states: Vec<Rc<RefCell<StateT>>>,
}

impl Propagator for PigeonPropagator {
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
            (*self.states[i]).borrow_mut().holes = vec![None; holes as usize];
        }
        true
    }

    fn propagate(&mut self, control: &mut PropagateControl, changes: &[SolverLiteral]) -> bool {
        // get the thread specific state
        let mut state = (*self.states[control.thread_id() as usize]).borrow_mut();

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
        let mut state = (*self.states[control.thread_id() as usize]).borrow_mut();

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

fn solve<P: Propagator>(
    ctl: GenericControl<CtrlCtx<P>>,
) -> Result<(Vec<Vec<String>>, GenericControl<CtrlCtx<P>>), ClingoError> {
    let mut ret = Vec::<Vec<String>>::new();
    // get a solve handle
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed to retrieve solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // get the model
            Ok(Some(model)) => ret.push(string_model(model)),
            // stop if there are no more models
            Ok(None) => break,
            Err(e) => return Err(e), // panic!("Error: {}", e),
        }
    }
    // close the solve handle
    let ctl = handle.close().expect("Failed to close solve handle.");
    Ok((ret, ctl))
}
fn string_model(model: &Model) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    // retrieve the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");
    for symbol in atoms {
        ret.push(symbol.to_string());
    }
    ret
}

#[test_case(2, 2, 2; "sat")]
#[test_case(5, 6, 0; "unsat")]
fn pigeon_propagator(holes: i32, pigeons: i32, number_of_models: usize) {
    // create a control context with propagator
    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        propagator: PigeonPropagator {
            pigeons: vec![],
            states: vec![],
        },
    };
    let mut ctl = control_with_context(vec!["0".into()], ctrl_ctx).unwrap();

    ctl.add(
        "pigeon",
        &vec!["h", "p"],
        "1 { place(P,H) : H = 1..h } 1 :- P = 1..p.",
    )
    .expect("Failed to add a logic program.");

    let arg0 = Symbol::create_number(holes);
    let arg1 = Symbol::create_number(pigeons);
    let args = vec![arg0, arg1];
    let part = Part::new("pigeon", args).unwrap();
    let parts = vec![part];

    // let place = |p, h| {
    //     Symbol::create_function(
    //         "place",
    //         &[Symbol::create_number(p), Symbol::create_number(h)],
    //         true,
    //     )
    //     .unwrap()
    // };

    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let (models, _) = solve(ctl).unwrap();
    assert_eq!(models.len(), number_of_models);
}

struct TestAssignment {
    a: Option<SolverLiteral>,
    b: Option<SolverLiteral>,
    c: Option<SolverLiteral>,
    count: usize,
}
impl Propagator for TestAssignment {
    fn init(&mut self, init: &mut PropagateInit) -> bool {
        let a1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "a")
            .unwrap()
            .literal()
            .unwrap();
        self.a = Some(init.solver_literal(a1).unwrap());
        let b1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "b")
            .unwrap()
            .literal()
            .unwrap();
        self.b = Some(init.solver_literal(b1).unwrap());
        let c1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "c")
            .unwrap()
            .literal()
            .unwrap();
        self.c = Some(init.solver_literal(c1).unwrap());

        init.add_watch(self.a.unwrap()).unwrap();
        init.add_watch(self.b.unwrap()).unwrap();
        true
    }
    fn propagate(&mut self, ctl: &mut PropagateControl, changes: &[SolverLiteral]) -> bool {
        let ass = ctl.assignment().unwrap();
        self.count += changes.len();
        let a_ = self.a.unwrap();
        let b_ = self.b.unwrap();
        let c_ = self.c.unwrap();
        assert!(ass.is_fixed(c_).unwrap());
        assert!(!ass.is_fixed(a_).unwrap());
        assert!(!ass.is_fixed(b_).unwrap());
        assert!(!ass.has_conflict());
        assert!(ass.has_literal(a_));
        assert!(ass.has_literal(b_));
        // REQUIRE(!ass.has_literal(1000));
        let decision = ass.decision(ass.decision_level()).unwrap();
        assert_eq!(ass.level(decision).unwrap(), ass.decision_level());
        if self.count == 1 {
            let a = changes[0];
            assert_eq!(changes.len(), 1);
            assert!(!ass.is_fixed(a_).unwrap());
            assert!(ass.is_true(a).unwrap());
            assert_eq!(ass.truth_value(a).unwrap(), TruthValue::True);
            assert!(ass.is_true(a_).unwrap() ^ ass.is_true(b_).unwrap());
            assert_eq!(ass.level(a).unwrap(), ass.decision_level());
        }
        if self.count == 2 {
            assert!(!ass.is_fixed(a_).unwrap());
            assert!(!ass.is_fixed(b_).unwrap());
            assert!(ass.is_true(a_).unwrap());
            assert!(ass.is_true(b_).unwrap());
        }
        true
    }
    fn undo(&mut self, _ctl: &mut PropagateControl, undo: &[SolverLiteral]) {
        self.count -= undo.len();
    }
}

#[test]
fn assignment_propagator() {
    // create a control context with propagator
    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        propagator: TestAssignment {
            a: None,
            b: None,
            c: None,
            count: 0,
        },
    };
    let mut ctl = control_with_context(vec!["0".into()], ctrl_ctx).unwrap();

    ctl.add("base", &[], "{a; b}. c.")
        .expect("Failed to add a logic program.");

    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let (models, _) = solve(ctl).unwrap();
    assert_eq!(models.len(), 4);
}

struct TestMode {
    lits: Vec<SolverLiteral>,
}
impl Propagator for TestMode {
    fn init(&mut self, init: &mut PropagateInit) -> bool {
        let atoms = init.symbolic_atoms().unwrap();
        let sig = Signature::new("p", 1, true).unwrap();
        for atom in atoms.iter_with_signature(sig).unwrap() {
            self.lits
                .push(init.solver_literal(atom.literal().unwrap()).unwrap());
        }
        init.set_check_mode(PropagatorCheckMode::Fixpoint);
        true
    }
    fn check(&mut self, ctl: &mut PropagateControl) -> bool {
        for lit in &self.lits {
            if ctl.assignment().unwrap().truth_value(*lit).unwrap() == TruthValue::Free {
                let _res = ctl.add_clause(&[*lit], ClauseType::Learnt);
                break;
            }
        }
        true
    }
}

#[test]
fn mode_propagator() {
    // create a control context with propagator
    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        propagator: TestMode { lits: vec![] },
    };
    let mut ctl = control_with_context(vec!["0".into()], ctrl_ctx).unwrap();

    ctl.add("base", &[], "{p(1..9)}.")
        .expect("Failed to add a logic program.");

    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let (models, _) = solve(ctl).unwrap();
    assert_eq!(
        models,
        [["p(1)", "p(2)", "p(3)", "p(4)", "p(5)", "p(6)", "p(7)", "p(8)", "p(9)"]]
    );
}

use std::collections::HashSet;
use std::sync::{Condvar, Mutex};
struct TestAddWatch {
    propagated: HashSet<SolverLiteral>,
    a: Option<SolverLiteral>,
    b: Option<SolverLiteral>,
}
struct TestPropagator {
    inner: Rc<RefCell<TestAddWatch>>,
    mutex: Mutex<u32>,
    cv: Condvar,
    done: bool,
}
impl Propagator for TestPropagator {
    fn init(&mut self, init: &mut PropagateInit) -> bool {
        assert_eq!(init.number_of_threads(), 2);
        let a1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "a")
            .unwrap()
            .literal()
            .unwrap();
        (*self.inner).borrow_mut().a = Some(init.solver_literal(a1).unwrap());
        let b1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "b")
            .unwrap()
            .literal()
            .unwrap();
        (*self.inner).borrow_mut().b = Some(init.solver_literal(b1).unwrap());
        let c1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "c")
            .unwrap()
            .literal()
            .unwrap();
        let c = init.solver_literal(c1).unwrap();
        let d1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "d")
            .unwrap()
            .literal()
            .unwrap();
        let d = init.solver_literal(d1).unwrap();
        let a_ = self.inner.borrow().a.unwrap();
        let b_ = self.inner.borrow().b.unwrap();
        init.add_watch_to_thread(a_, 0).unwrap();
        init.add_watch_to_thread(a_.negate(), 0).unwrap();
        init.add_watch_to_thread(b_, 0).unwrap();
        init.add_watch_to_thread(b_.negate(), 0).unwrap();
        init.add_watch_to_thread(b_.negate(), 1).unwrap();
        init.add_watch_to_thread(b_, 1).unwrap();
        let assignment = init.assignment().unwrap();
        assert_eq!(assignment.truth_value(a_).unwrap(), TruthValue::Free);
        assert_eq!(assignment.truth_value(b_).unwrap(), TruthValue::Free);
        assert_eq!(assignment.truth_value(c).unwrap(), TruthValue::True);
        assert_eq!(assignment.truth_value(d).unwrap(), TruthValue::False);
        self.done = false;
        true
    }
    fn propagate(&mut self, ctl: &mut PropagateControl, changes: &[SolverLiteral]) -> bool {
        if ctl.thread_id() == 0 {
            // wait for thread 1 to propagate b
            while !self.done {
                let _mut_ = self.cv.wait(self.mutex.lock().unwrap()).unwrap();
            }
        } else {
            let mut s = (*self.inner).borrow_mut();
            for lit in changes {
                let _mut_ = self.mutex.lock().unwrap();
                self.done = true;
                if lit.get_integer() < 0 {
                    s.propagated.insert(lit.negate());
                } else {
                    s.propagated.insert(*lit);
                }
            }
            self.cv.notify_one();
        }
        true
    }
}

#[test]
fn add_watch_propagator() {
    let p = Rc::new(RefCell::new(TestAddWatch {
        propagated: HashSet::new(),
        a: None,
        b: None,
    }));

    // create a control context with propagator
    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        propagator: TestPropagator {
            inner: p.clone(),
            mutex: Mutex::new(0),
            cv: Condvar::new(),
            done: false,
        },
    };
    let mut ctl = control_with_context(vec!["0".into()], ctrl_ctx).unwrap();

    let conf = ctl.configuration_mut().unwrap();
    let root_key = conf.root().unwrap();
    let sub_key = conf.map_at(root_key, "solve.parallel_mode").unwrap();
    conf.value_set(sub_key, "2")
        .expect("Failed to set solve.parallel_mode to 2.");
    ctl.add("base", &[], "{a;b;c;d}. c. :- d.")
        .expect("Failed to add a logic program.");

    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let (mut models, _) = solve(ctl).unwrap();
    models.sort();
    assert_eq!(
        models,
        [
            vec!["a", "b", "c"],
            vec!["a", "c"],
            vec!["b", "c"],
            vec!["c"],
        ]
    );
    let b = p.borrow().b.unwrap();
    let mut test_set = HashSet::new();
    test_set.insert(b);
    assert_eq!(p.borrow().propagated, test_set);
}

struct TestAddClause {
    clause_type: ClauseType,
    enable: bool,
    a: Option<SolverLiteral>,
    b: Option<SolverLiteral>,
    count: usize,
}
struct TestPropagator2 {
    inner: Rc<RefCell<TestAddClause>>,
}
impl Propagator for TestPropagator2 {
    fn init(&mut self, init: &mut PropagateInit) -> bool {
        let mut s = (*self.inner).borrow_mut();
        let a1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "a")
            .unwrap()
            .literal()
            .unwrap();
        s.a = Some(init.solver_literal(a1).unwrap());
        let b1 = init
            .symbolic_atoms()
            .unwrap()
            .iter()
            .unwrap()
            .find(|x| x.symbol().unwrap().to_string() == "b")
            .unwrap()
            .literal()
            .unwrap();
        s.b = Some(init.solver_literal(b1).unwrap());
        init.add_watch(s.a.unwrap()).unwrap();
        init.add_watch(s.b.unwrap()).unwrap();
        true
    }
    fn propagate(&mut self, ctl: &mut PropagateControl, changes: &[SolverLiteral]) -> bool {
        let mut s = (*self.inner).borrow_mut();
        s.count += changes.len();
        if s.enable && s.count == 2 {
            let b1 = ctl
                .add_clause(
                    &[s.a.unwrap().negate(), s.b.unwrap().negate()],
                    s.clause_type,
                )
                .unwrap();
            let b2 = ctl.propagate().unwrap();
            if b1 && b2 {
                return false;
            }
        }
        true
    }
    fn undo(&mut self, _ctl: &mut PropagateControl, undo: &[SolverLiteral]) {
        let mut s = (*self.inner).borrow_mut();
        s.count -= undo.len();
    }
}

#[test_case(ClauseType::Learnt, 3, 4; "learnt")]
#[test_case(ClauseType::Static, 3, 3; "static ")]
#[test_case(ClauseType::Volatile, 3, 4; "volatile")]
#[test_case(ClauseType::VolatileStatic, 3, 4; "volatile_static")]
fn add_clause(clause_type: ClauseType, m1: usize, m2: usize) {
    let data = Rc::new(RefCell::new(TestAddClause {
        clause_type,
        enable: true,
        a: None,
        b: None,
        count: 0,
    }));

    // create a control context with propagator
    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        propagator: TestPropagator2 {
            inner: data.clone(),
        },
    };
    let mut ctl = control_with_context(vec!["0".into()], ctrl_ctx).unwrap();

    ctl.add("base", &[], "{a; b}.")
        .expect("Failed to add a logic program.");

    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let (models, ctl) = solve(ctl).unwrap();

    assert_eq!(models.len(), m1);
    (*data).borrow_mut().enable = false;
    let (models, _) = solve(ctl).unwrap();
    assert_eq!(models.len(), m2);
}
