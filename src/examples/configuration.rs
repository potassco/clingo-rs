use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;


extern "C" fn on_model(model: &mut ClingoModel, data: *mut c_void, goon: *mut u8) -> u8 {

    // retrieve the symbols in the model
    let atoms = model.symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

    print!("Model:");
    for atom in atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        print!(" {}", atom_string.to_str().unwrap());
    }
    println!("");
    return 1;
}

fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn main() {

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // get the configuration object and its root key
    {
        let conf = ctl.configuration().unwrap();
        let root_key = conf.configuration_root().unwrap();

        // configure to enumerate all models
        let mut sub_key = conf.configuration_map_at(root_key, "solve.models").unwrap();
        let err1 = conf.configuration_value_set(sub_key, "0");
        if err1 == 0 {
            return error_main();
        }
        sub_key = conf.configuration_map_at(root_key, "solver").unwrap();

        // configure the first solver to use the berkmin heuristic
        sub_key = conf.configuration_array_at(sub_key, 0).unwrap();
        sub_key = conf.configuration_map_at(sub_key, "heuristic").unwrap();
        let err2 = conf.configuration_value_set(sub_key, "berkmin");
        if err2 == 0 {
            return error_main();
        }
    }
    // note that the solver entry can be used both as an array and a map
    // if used as a map, this simply sets the configuration of the first solver and
    // is equivalent to the code above

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err3 = ctl.add("base", parameters, "a :- not b. b :- not a.");
    if err3 == 0 {
        return error_main();
    }

    println!("");

    // ground the base part
    let part = ClingoPart {
        name: CString::new("base").unwrap(),      
        params: &[],
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err4 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err4 == 0 {
        return error_main();
    }

    // solve using a model callback
    let solve_callback: ClingoModelCallback = Some(on_model);
    let solve_callback_data = std::ptr::null_mut();
    let assumptions = vec![];
    let _solve_result = ctl.solve(solve_callback, solve_callback_data, assumptions)
        .expect("Failed while solving");

}
