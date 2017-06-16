extern crate clingo;

use std::env;
use std::ffi::CString;
use clingo::*;


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn print_prefix(depth: u8) {
    for _ in 0..depth {
        print!("  ");
    }
}

// recursively print the statistics object
fn print_statistics(stats: &mut ClingoStatistics, key: u64, depth: u8) {

    // get the type of an entry and switch over its various values
    let statistics_type = stats.statistics_type(key).unwrap();
    match statistics_type {
        // print values
        1 => {
            let value = stats.statistics_value_get(key).expect(
                "Failed to retrieve statistics value",
            );

            // print value (with prefix for readability)
            print_prefix(depth);
            println!("{}", value);
        }

        // print arrays
        2 => {
            // loop over array elements
            let size = stats.statistics_array_size(key).expect(
                "Failed to retrieve statistics array size",
            );
            for i in 0..size {

                // print array offset (with prefix for readability)
                let subkey = stats.statistics_array_at(key, i).expect(
                    "Failed to retrieve statistics array at _",
                );
                print_prefix(depth);
                println!("{} zu:", i);

                // recursively print subentry
                print_statistics(stats, subkey, depth + 1);
            }
        }

        // print maps
        3 => {
            // loop over map elements
            let size = stats.statistics_map_size(key).unwrap();
            for i in 0..size {
                // get and print map name (with prefix for readability)
                let name = stats.statistics_map_subkey_name(key, i).unwrap();
                let subkey = stats.statistics_map_at(key, name).unwrap();
                print_prefix(depth);
                print!("{}", name);

                // recursively print subentry
                print_statistics(stats, subkey, depth + 1);
            }
        }

        // this case won't occur if the statistics are traversed like this
        _ => {
            println!("clingo_statistics_type_empty");
        }
    }
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(
            clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t,
        )
        .expect("Failed to retrieve symbols in the model");

    print!("Model:");

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

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // get the configuration object and its root key
    {
        let conf = ctl.configuration().unwrap();
        let root_key = conf.configuration_root().unwrap();
        // and set the statistics level to one to get more statistics
        let subkey = conf.configuration_map_at(root_key, "stats").unwrap();
        let err = conf.configuration_value_set(subkey, "1");
        if !err {
            return error_main();
        }
    }

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err = ctl.add("base", parameters, "a :- not b. b :- not a.");
    if !err {
        return error_main();
    }

    print!("");

    // ground the base part
    let part = ClingoPart {
        name: CString::new("base").unwrap(),
        params: &[],
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err = ctl.ground(parts, ground_callback, ground_callback_data);
    if !err {
        return error_main();
    }

    // solve
    let _solve_result = solve(ctl);

    // get the statistics object, get the root key, then print the statistics recursively
    let mut stats = ctl.statistics().unwrap();
    let stats_key = stats.statistics_root().unwrap();
    print_statistics(stats, stats_key, 0);
}
