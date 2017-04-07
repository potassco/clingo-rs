use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;


fn print_model(model: *mut clingo_model_t,
               data: *mut c_void,
               label: &str,
               show: clingo_show_type_bitset_t) {

    print!("{}:", label);

    // retrieve the symbols in the model
    let atoms = safe_clingo_model_symbols(model, show)
        .expect("Failed to retrieve symbols in the model");

    for atom in &atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        print!(" {}", atom_string.to_str().unwrap());
    }
    println!("");
}

extern "C" fn on_model(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {

    // get model type
    let model_type = safe_clingo_model_type(model).unwrap();

    let clingo_model_type_stable_model =
        clingo_model_type::clingo_model_type_stable_model as clingo_model_type_t;
    let clingo_model_type_brave_consequences =
        clingo_model_type::clingo_model_type_brave_consequences as clingo_model_type_t;
    let clingo_model_type_cautious_consequences =
        clingo_model_type::clingo_model_type_cautious_consequences as clingo_model_type_t;

    let mut type_string = "";
    match model_type {
        clingo_model_type_stable_model => type_string = "Stable model",
        clingo_model_type_brave_consequences => type_string = "Brave consequences", 
        clingo_model_type_cautious_consequences => type_string = "Cautious consequences",
    };

    // get running number of model
    let number = safe_clingo_model_number(model).unwrap();

    println!("{}: {}", type_string, number);

    print_model(model,
                data,
                "  shown",
                clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t);
    print_model(model,
                data,
                "  atoms",
                clingo_show_type::clingo_show_type_atoms as clingo_show_type_bitset_t);
    print_model(model,
                data,
                "  terms",
                clingo_show_type::clingo_show_type_terms as clingo_show_type_bitset_t);
    print_model(model,
                data,
                " ~atoms",
                (clingo_show_type::clingo_show_type_complement as clingo_show_type_bitset_t +
                 clingo_show_type::clingo_show_type_atoms as clingo_show_type_bitset_t));

    // continue solving after a model has been reported
    //     unsafe {
    //         *goon = 1;
    //     }
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
    let mut ctl = SafeClingoControl::new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, "1 {a; b} 1. #show c : b. #show a/0.");
    if err1 == 0 {
        return error_main();
    }
    println!("");

    // ground the base part
    let part = safe_clingo_part {
        params: 0,
        size: 0,
        name: CString::new("base").unwrap(),
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err2 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err2 == 0 {
        return error_main();
    }

    // solve using a model callback
    let solve_callback: clingo_model_callback_t = Some(on_model);
    let solve_callback_data = std::ptr::null_mut();
    let assumptions = vec![];
    let solve_result = ctl.solve(solve_callback, solve_callback_data, assumptions)
        .expect("Failed while solving");

}
