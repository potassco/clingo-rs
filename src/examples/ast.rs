extern crate clingo;

use std::env;
use std::ffi::CString;
use clingo::*;


pub struct OnStatementData {
    atom: clingo_ast_term_t,
    // builder: &'a mut ClingoProgramBuilder,
}

// adds atom enable to all rule bodies
extern "C" fn on_statement(stm: *const clingo_ast_statement_t,
                           data: *mut std::os::raw::c_void)
                           -> bool {
    let ret = true;
    //   clingo_ast_rule_t rule;
    //   clingo_ast_body_literal_t *body = NULL;
    //   clingo_ast_literal_t lit;
    //   clingo_ast_statement_t stm2;

    // let builder = (*data).builder;

    // pass through all statements that are not rules
    // if (*stm).type != clingo_ast_statement_type::clingo_ast_statement_type_rule {
    //     if !builder.add(*stm) {
    //         return error_main();
    //     }
    //     return ret;
    // }

    // allocate space to hold the current rule body + one literal
    //   body = (clingo_ast_body_literal_t*)malloc(sizeof(clingo_ast_body_literal_t) * (stm->rule->size + 1));
    //   if (!body) {
    //     clingo_set_error(clingo_error_bad_alloc, "could not allocate memory for rule body");
    //     return error_main();
    //   }

    // copy the current rule body
    //   for (size_t i = 0; i < stm->rule->size; ++i) {
    //     body[i] = stm->rule->body[i];
    //   }

    // create atom enable
    //   lit.symbol   = &data->atom;
    //   lit.location = data->atom.location;
    //   lit.type     = clingo_ast_literal_type_symbolic;
    //   lit.sign     = clingo_ast_sign_none;

    // add atom enable to the rule body
    //   body[stm->rule->size].location = data->atom.location;
    //   body[stm->rule->size].type     = clingo_ast_body_literal_type_literal;
    //   body[stm->rule->size].sign     = clingo_ast_sign_none;
    //   body[stm->rule->size].literal  = &lit;

    // initialize the rule
    //   rule.head = stm->rule->head;
    //   rule.size = stm->rule->size + 1;
    //   rule.body = body;

    // initialize the statement
    //   stm2.location = stm->location;
    //   stm2.type     = stm->type;
    //   stm2.rule     = &rule;

    // add the rewritten statement to the program
    //   if (!clingo_program_builder_add(data->builder, &stm2)) { return error_main(); }

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
        .symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

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
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    let sym = safe_clingo_symbol_create_id("enable", true).unwrap();

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
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 {
            symbol: __BindgenUnionField::new(),
            variable: __BindgenUnionField::new(),
            unary_operation: __BindgenUnionField::new(),
            binary_operation: __BindgenUnionField::new(),
            interval: __BindgenUnionField::new(),
            function: __BindgenUnionField::new(),
            external_function: __BindgenUnionField::new(),
            pool: __BindgenUnionField::new(),
            bindgen_union_field: sym,
        };
        let atom = clingo_ast_term_t {
            location: location,
            type_: clingo_ast_term_type::clingo_ast_term_type_symbol as clingo_ast_term_type_t,
            __bindgen_anon_1: _bg_union_1,
        };
        let mut data = OnStatementData {
            atom: atom,
            // builder: builder,
        };

        // begin building a program
        if !builder.begin() {
            return error_main();
        }

        // get the AST of the program
        let logger = None;
        let logger_data = std::ptr::null_mut();
        let callback: clingo_ast_callback_t = Some(on_statement);
        let data_ptr =
            unsafe { std::mem::transmute::<&OnStatementData, *mut ::std::os::raw::c_void>(&data) };
        if !safe_clingo_parse_program("a :- not b. b :- not a.",
                                      callback,
                                      data_ptr,
                                      logger,
                                      logger_data,
                                      20) {
            return error_main();
        }

        // add the external statement: #external enable.
        let ext = clingo_ast_external {
            atom: data.atom,
            body: std::ptr::null(),
            size: 0,
        };
        let stm =
            ClingoAstStatement::new(location,
                                    clingo_ast_statement_type::clingo_ast_statement_type_external,
                                    &ext);
        if !builder.add(&stm) {
            return error_main();
        }

        // finish building a program
        if !builder.end() {
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
