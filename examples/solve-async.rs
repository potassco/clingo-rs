use clingo::*;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use std::env;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

struct MySEHandler {
    atom: Rc<AtomicBool>,
}
impl SolveEventHandler for MySEHandler {
    fn on_solve_event(&mut self, event: SolveEvent, _goon: &mut bool) -> bool {
        if let SolveEvent::Finish(_result) = event {
            self.atom.store(false, Ordering::SeqCst);
        }
        true
    }
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = control(options).expect("Failed creating Control.");

    // add a logic program to the base part
    ctl.add(
        "base",
        &[],
        "#const n = 17.\
         1 { p(X); q(X) } 1 :- X = 1..n.\
         :- not n+1 { p(1..n); \
         q(1..n) }.",
    )
    .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let running = Rc::new(AtomicBool::new(true));

    // create a solve handle with an attached event handler
    let mut handle = ctl
        .solve_with_event_handler(
            SolveMode::ASYNC | SolveMode::YIELD,
            &[],
            MySEHandler {
                atom: running.clone(),
            },
        )
        .expect("Failed to retrieve solve handle.");

    // let's approximate pi
    let mut samples = 0.;
    let mut in_circle = 0.;
    let mut rng = rand::thread_rng();
    let between = Uniform::new_inclusive(-1f64, 1.);
    while running.load(Ordering::SeqCst) {
        samples += 1.;
        let x = between.sample(&mut rng);
        let y = between.sample(&mut rng);
        if x * x + y * y <= 1. {
            in_circle += 1.;
        }
    }
    println!("pi = {}", 4. * in_circle / samples);

    // get the solve result
    handle
        .get()
        .expect("Failed to get result from solve handle.");

    // close the handle
    handle.close().expect("Failed to close solve handle.");
}
