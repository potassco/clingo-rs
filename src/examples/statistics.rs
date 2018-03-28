extern crate clingo;

use std::env;
use clingo::*;

fn print_prefix(depth: u8) {
    for _ in 0..depth {
        print!("  ");
    }
}

// recursively print the statistics object
fn print_statistics(stats: &Statistics, key: u64, depth: u8) {
    // get the type of an entry and switch over its various values
    let statistics_type = stats.statistics_type(key).unwrap();
    match statistics_type {
        // print values
        StatisticsType::Value => {
            let value = stats
                .value_get(key)
                .expect("Failed to retrieve statistics value.");

            // print value (with prefix for readability)
            print_prefix(depth);
            println!("{}", value);
        }

        // print arrays
        StatisticsType::Array => {
            // loop over array elements
            let size = stats
                .array_size(key)
                .expect("Failed to retrieve statistics array size.");
            for i in 0..size {
                // print array offset (with prefix for readability)
                let subkey = stats
                    .statistics_array_at(key, i)
                    .expect("Failed to retrieve statistics array.");
                print_prefix(depth);
                println!("{} zu:", i);

                // recursively print subentry
                print_statistics(stats, subkey, depth + 1);
            }
        }

        // print maps
        StatisticsType::Map => {
            // loop over map elements
            let size = stats.map_size(key).unwrap();
            for i in 0..size {
                // get and print map name (with prefix for readability)
                let name = stats.map_subkey_name(key, i).unwrap();
                let subkey = stats.map_at(key, name).unwrap();
                print_prefix(depth);
                print!("{}", name);

                // recursively print subentry
                print_statistics(stats, subkey, depth + 1);
            }
        }

        // this case won't occur if the statistics are traversed like this
        StatisticsType::Empty => {
            println!("StatisticsType::Empty");
        }
    }
}

fn print_model(model: &Model) {
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
    let mut handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

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

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = Control::new(options).expect("Failed creating Control.");

    {
        // get the configuration object and its root key
        let conf = ctl.configuration_mut().unwrap();
        let root_key = conf.root().unwrap();

        // and set the statistics level to one to get more statistics
        let subkey = conf.map_at(root_key, "stats").unwrap();
        conf.value_set(subkey, "1")
            .expect("Failed to set value in configuration.");
    }

    // add a logic program to the base part
    ctl.add("base", &[], "a :- not b. b :- not a.").unwrap();

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts).unwrap();

    // solve
    solve(&mut ctl);

    // get the statistics object, get the root key, then print the statistics recursively
    let stats = ctl.statistics().unwrap();
    let stats_key = stats.root().unwrap();
    print_statistics(stats, stats_key, 0);
}
