extern crate clingo;

use std::env;
use std::ffi::CString;
use clingo::*;


pub struct OnStatementData<'a> {
    atom: ClingoAstTerm,
    builder: &'a mut ClingoProgramBuilder,
}

// adds atom enable to all rule bodies

extern "C" fn on_statement(
    stm_: *const clingo_ast_statement_t,
    data: *mut std::os::raw::c_void,
) -> bool {
    let ret = true;

    let stm = unsafe {
        (std::mem::transmute::<*const clingo_ast_statement_t, *const ClingoAstStatement>(stm_))
            .as_ref()
    }.unwrap();
    let mut on_statement_data = unsafe { (data as *mut OnStatementData).as_mut() }.unwrap();

    // pass through all statements that are not rules
    if stm.get_type() != clingo_ast_statement_type::clingo_ast_statement_type_rule {
        if !on_statement_data.builder.add(&stm) {
            // return error_main();
            return false;
        }
        return true;
    }

    // allocate space to hold the current rule body + one literal
    //   body = (clingo_ast_body_literal_t*)malloc(sizeof(clingo_ast_body_literal_t) * (stm->rule->size + 1));
    //   if (!body) {
    //     clingo_set_error(clingo_error_bad_alloc, "could not allocate memory for rule body");
    //     return error_main();
    //   }

    // copy the current rule body
    let mut body = std::vec::Vec::new();
    body.clone_from_slice(unsafe { stm.rule() }.body());

    // create atom enable
    let lit = ClingoAstLiteral::new(
        on_statement_data.atom.location(),
        clingo_ast_sign::clingo_ast_sign_none,
        clingo_ast_literal_type::clingo_ast_literal_type_symbolic,
        &on_statement_data.atom,
    );

    // add atom enable to the rule body
    let y: ClingoAstBodyLiteral = ClingoAstBodyLiteral::new(
        on_statement_data.atom.location(),
        clingo_ast_sign::clingo_ast_sign_none,
        clingo_ast_body_literal_type::clingo_ast_body_literal_type_literal,
        &lit,
    );
    body.push(y);

    // initialize the rule
    let rule = ClingoAstRule::new(unsafe { stm.rule() }.head(), &body);

    // initialize the statement
    let stm2 = ClingoAstStatement::new_rule(stm.location(), stm.get_type(), &rule);

    // add the rewritten statement to the program
    //   if (!clingo_program_builder_add(data->builder, &stm2)) { return error_main(); }
    if !on_statement_data.builder.add(&stm2) {
        // return error_main();
        return false;
    }

    //   goto out;
    // error:
    //   ret = false;
    // out:
    //   if (body) { free(body); }

    return ret;
}

fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
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

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    let sym = clingo_symbol::create_id("enable", true).unwrap();

    {
        // get the program builder
        let mut builder = ctl.program_builder().unwrap();

        // initialize the location
        let location = clingo_location {
            begin_line: 0,
            end_line: 0,
            begin_column: 0,
            end_column: 0,
            begin_file: CString::new("<rewrite>").unwrap().as_ptr(),
            end_file: CString::new("<rewrite>").unwrap().as_ptr(),
        };

        // initilize atom to add
        let atom = ClingoAstTerm::new_symbol(location, sym);

        let mut data = OnStatementData {
            atom: atom,
            builder: builder,
        };

        // begin building a program
        if !data.builder.begin() {
            return error_main();
        }

        // get the AST of the program
        let logger = None;
        let logger_data = std::ptr::null_mut();
        let callback: clingo_ast_callback_t = Some(on_statement);
        let data_ptr =
            unsafe { std::mem::transmute::<&OnStatementData, *mut ::std::os::raw::c_void>(&data) };
        if !safe_clingo_parse_program(
            "a :- not b. b :- not a.",
            callback,
            data_ptr,
            logger,
            logger_data,
            20,
        )
        {
            return error_main();
        }

        // add the external statement: #external enable.
        let ext = ClingoAstExternal::new(data.atom, &[]);

        let stm = ClingoAstStatement::new_external(
            location,
            clingo_ast_statement_type::clingo_ast_statement_type_external,
            &ext,
        );

        if !data.builder.add(&stm) {
            return error_main();
        }

        // finish building a program
        if !data.builder.end() {
            return error_main();
        }
    }

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

    // solve with external enable = false
    println!("Solving with enable = false...");
    solve(ctl);

    // solve with external enable = true
    println!("Solving with enable = true...");
    if !ctl.assign_external(sym, clingo_truth_value::clingo_truth_value_true) {
        return error_main();
    }
    solve(ctl);

    // solve with external enable = false
    println!("Solving with enable = false...");
    if !ctl.assign_external(sym, clingo_truth_value::clingo_truth_value_false) {
        return error_main();
    }
    solve(ctl);
}
