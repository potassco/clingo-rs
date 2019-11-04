use clingo::*;
use std::env;

fn print_prefix(depth: u8) {
    println!();
    for _ in 0..depth {
        print!("  ");
    }
}

// recursively print the configuartion object
fn print_configuration(conf: &Configuration, key: Id, depth: u8) {
    // get the type of an entry and switch over its various values
    let configuration_type = conf.configuration_type(key).unwrap();
    match configuration_type {
        // print values
        ConfigurationType::VALUE => {
            let value = conf
                .value_get(key)
                .expect("Failed to retrieve statistics value.");

            println!("{}", value);
        }

        // print arrays
        ConfigurationType::ARRAY => {
            // loop over array elements
            let size = conf
                .array_size(key)
                .expect("Failed to retrieve statistics array size.");
            for i in 0..size {
                // print array offset (with prefix for readability)
                let subkey = conf
                    .array_at(key, i)
                    .expect("Failed to retrieve statistics array.");
                print_prefix(depth);
                print!("{}:", i);

                // recursively print subentry
                print_configuration(conf, subkey, depth + 1);
            }
        }

        // print maps
        ConfigurationType::MAP => {
            // loop over map elements
            let size = conf.map_size(key).unwrap();
            for i in 0..size {
                // get and print map name (with prefix for readability)
                let name = conf.map_subkey_name(key, i).unwrap();
                let subkey = conf.map_at(key, name).unwrap();
                print_prefix(depth);
                print!("{}:", name);

                // recursively print subentry
                print_configuration(conf, subkey, depth + 1);
            }
        }

        // this case won't occur if the configuration are traversed like this
        _ => {
            let bla = conf.value_get(key).unwrap();
            print!(" {}", bla);
            // println!("Unknown ConfigurationType");
        }
    }
}

fn print_model(model: &Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
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
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

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
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = Control::new(options).expect("Failed creating Control.");

    {
        // get the configuration object and its root key
        let conf = ctl.configuration_mut().unwrap();
        let root_key = conf.root().unwrap();

        print_configuration(conf, root_key, 0);
        let mut sub_key;

        // configure to enumerate all models
        sub_key = conf.map_at(root_key, "solve.models").unwrap();
        conf.value_set(sub_key, "0")
            .expect("Failed to set solve.models to 0.");

        // configure the first solver to use the berkmin heuristic
        sub_key = conf.map_at(root_key, "solver").unwrap();
        sub_key = conf.array_at(sub_key, 0).unwrap();
        sub_key = conf.map_at(sub_key, "heuristic").unwrap();
        conf.value_set(sub_key, "berkmin")
            .expect("Failed to set heuristic to berkmin.");
    }
    // note that the solver entry can be used both as an array and a map
    // if used as a map, this simply sets the configuration of the first solver and
    // is equivalent to the code above

    // add a logic program to the base part
    ctl.add("base", &[], "a :- not b. b :- not a.")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve
    solve(&mut ctl);
}
