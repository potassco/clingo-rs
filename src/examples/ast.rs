use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;


extern "C" fn on_model(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {

    // retrieve the symbols in the model
    let atoms = safe_clingo_model_symbols(model, clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

    println!("Model:");
    for atom in &atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        println!(" {}", atom_string.to_str().unwrap());
    }
    return 1;
}


pub struct on_statement_data{
  atom: clingo_ast_term_t,
  builder: *clingo_program_builder_t;
}

// adds atom enable to all rule bodies
extern "C" fn on_statement (stm: *const clingo_ast_statement_t, data: *mut on_statement_data) -> bool {
  let ret = true;
//   clingo_ast_rule_t rule;
//   clingo_ast_body_literal_t *body = NULL;
//   clingo_ast_literal_t lit;
//   clingo_ast_statement_t stm2;

  // pass through all statements that are not rules
  if (stm->type != clingo_ast_statement_type_rule) {
    if (!clingo_program_builder_add(data->builder, stm)) { goto error; }
    goto out;
  }

  // allocate space to hold the current rule body + one literal
  body = (clingo_ast_body_literal_t*)malloc(sizeof(clingo_ast_body_literal_t) * (stm->rule->size + 1));
  if (!body) {
    clingo_set_error(clingo_error_bad_alloc, "could not allocate memory for rule body");
    goto error;
  }

  // copy the current rule body
  for (size_t i = 0; i < stm->rule->size; ++i) {
    body[i] = stm->rule->body[i];
  }

  // create atom enable
  lit.symbol   = &data->atom;
  lit.location = data->atom.location;
  lit.type     = clingo_ast_literal_type_symbolic;
  lit.sign     = clingo_ast_sign_none;

  // add atom enable to the rule body
  body[stm->rule->size].location = data->atom.location;
  body[stm->rule->size].type     = clingo_ast_body_literal_type_literal;
  body[stm->rule->size].sign     = clingo_ast_sign_none;
  body[stm->rule->size].literal  = &lit;

  // initialize the rule
  rule.head = stm->rule->head;
  rule.size = stm->rule->size + 1;
  rule.body = body;

  // initialize the statement
  stm2.location = stm->location;
  stm2.type     = stm->type;
  stm2.rule     = &rule;

  // add the rewritten statement to the program
  if (!clingo_program_builder_add(data->builder, &stm2)) { goto error; }

  goto out;
error:
  ret = false;
out:
  if (body) { free(body); }

  return ret;
}

fn main() {
//   char const *error_message;
//   int ret = 0;
//   clingo_solve_result_bitset_t solve_ret;
//   clingo_control_t *ctl = NULL;
//   clingo_symbol_t sym;
//   clingo_location_t location;
//   clingo_ast_statement_t stm;
//   clingo_ast_external_t ext;
//   on_statement_data data;
//   clingo_part_t parts[] = {{ "base", NULL, 0 }};

  // create a control object and pass command line arguments
  if (!clingo_control_new(argv+1, argc-1, NULL, NULL, 20, &ctl) != 0) { goto error; }

  // get the program builder
  if (!clingo_control_program_builder(ctl, &data.builder)) { goto error; }

  // initialize the location
  location.begin_line   = location.end_line   = 0;
  location.begin_column = location.end_column = 0;
  location.begin_file   = location.end_file   = "<rewrite>";

  // initilize atom to add
  if (!clingo_symbol_create_id("enable", true, &sym)) { goto error; }
  data.atom.location = location;
  data.atom.type = clingo_ast_term_type_symbol;
  data.atom.symbol = sym;

  // begin building a program
  if (!clingo_program_builder_begin(data.builder)) { goto error; }

  // get the AST of the program
  if (!clingo_parse_program("a :- not b. b :- not a.", (clingo_ast_callback_t*)on_statement, &data, NULL, NULL, 20)) { goto error; }

  // add the external statement: #external enable.
  ext.atom = data.atom;
  ext.body = NULL;
  ext.size = 0;
  stm.location = location;
  stm.type = clingo_ast_statement_type_external;
  stm.external = &ext;
  if (!clingo_program_builder_add(data.builder, &stm)) { goto error; }

  // finish building a program
  if (!clingo_program_builder_end(data.builder)) { goto error; }

  // ground the base part
  if (!clingo_control_ground(ctl, parts, 1, NULL, NULL)) { goto error; }

  // solve with external enable = false
  printf("Solving with enable = false...\n");
  if (!clingo_control_solve(ctl, on_model, NULL, NULL, 0, &solve_ret)) { goto error; }
  // solve with external enable = true
  printf("Solving with enable = true...\n");
  if (!clingo_control_assign_external(ctl, sym, clingo_truth_value_true)) { goto error; }
  if (!clingo_control_solve(ctl, on_model, NULL, NULL, 0, &solve_ret)) { goto error; }
  // solve with external enable = false
  printf("Solving with enable = false...\n");
  if (!clingo_control_assign_external(ctl, sym, clingo_truth_value_false)) { goto error; }
  if (!clingo_control_solve(ctl, on_model, NULL, NULL, 0, &solve_ret)) { goto error; }

  goto out;

error:
  if (!(error_message = clingo_error_message())) { error_message = "error"; }

  printf("%s\n", error_message);
  ret = clingo_error_code();

out:
  if (ctl) { clingo_control_free(ctl); }

  return ret;
}

