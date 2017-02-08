extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;

fn main() {

// ar rc build/release/libgringo.a build/release/libgringo/src/control.o build/release/libgringo/src/term.o build/release/libgringo/src/symbol.o build/release/libgringo/src/primes.o build/release/libgringo/src/backend.o build/release/libgringo/src/lua.o build/release/libgringo/src/scripts.o build/release/libgringo/src/python.o build/release/libgringo/src/terms.o build/release/libgringo/src/ground/statements.o build/release/libgringo/src/ground/literals.o build/release/libgringo/src/ground/instantiation.o build/release/libgringo/src/ground/program.o build/release/libgringo/src/input/groundtermgrammar/grammar.o build/release/libgringo/src/input/aggregate.o build/release/libgringo/src/input/literals.o build/release/libgringo/src/input/groundtermparser.o build/release/libgringo/src/input/nongroundgrammar/grammar.o build/release/libgringo/src/input/nongroundparser.o build/release/libgringo/src/input/aggregates.o build/release/libgringo/src/input/program.o build/release/libgringo/src/input/programbuilder.o build/release/libgringo/src/input/statement.o build/release/libgringo/src/input/literal.o build/release/libgringo/src/input/theory.o build/release/libgringo/src/output/statements.o build/release/libgringo/src/output/literals.o build/release/libgringo/src/output/aggregates.o build/release/libgringo/src/output/output.o build/release/libgringo/src/output/statement.o build/release/libgringo/src/output/literal.o build/release/libgringo/src/output/theory.o

  gcc::Config::new()
        .cpp(true)
        .define("NDEBUG",Some("1"))
        .file("clingo/libgringo/src/control.cc")
        .file("clingo/libgringo/src/term.cc")
        .file("clingo/libgringo/src/symbol.cc")
        .file("clingo/libgringo/src/primes.cc")
        .file("clingo/libgringo/src/backend.cc")
        .file("clingo/libgringo/src/lua.cc")
        .file("clingo/libgringo/src/scripts.cc")
        .file("clingo/libgringo/src/python.cc")
        .file("clingo/libgringo/src/terms.cc")
        .file("clingo/libgringo/src/ground/statements.cc")
        .file("clingo/libgringo/src/ground/literals.cc")
        .file("clingo/libgringo/src/ground/instantiation.cc")       
        .file("clingo/libgringo/src/input/groundtermparser.cc")
        .file("clingo/build/release/libgringo/src/input/nongroundgrammar/grammar.cc")
        .file("clingo/build/release/libgringo/src/input/groundtermgrammar/grammar.cc") //
        .file("clingo/libgringo/src/input/nongroundparser.cc")
        .file("clingo/libgringo/src/input/aggregate.cc")
        .file("clingo/libgringo/src/input/aggregates.cc")
        .file("clingo/libgringo/src/input/program.cc")
        .file("clingo/libgringo/src/ground/program.cc") //
        .file("clingo/libgringo/src/input/programbuilder.cc")
        .file("clingo/libgringo/src/input/literals.cc") //
        .file("clingo/libgringo/src/input/statement.cc")
        .file("clingo/libgringo/src/input/literal.cc")
        .file("clingo/libgringo/src/input/theory.cc")
        .file("clingo/libgringo/src/output/statements.cc")
        .file("clingo/libgringo/src/output/literals.cc")
        .file("clingo/libgringo/src/output/aggregates.cc")
        .file("clingo/libgringo/src/output/output.cc")
        .file("clingo/libgringo/src/output/statement.cc")
        .file("clingo/libgringo/src/output/literal.cc")
        .file("clingo/libgringo/src/output/theory.cc")
        .include("clingo/libgringo")
        .include("clingo/build/release/libgringo/src")
        .include("clingo/liblp")     
        .include("clingo/libreify")
        .compile("libgringo.a");

// ar rc build/release/libprogram_opts.a build/release/libprogram_opts/src/application.o build/release/libprogram_opts/src/alarm.o build/release/libprogram_opts/src/program_options.o build/release/libprogram_opts/src/string_convert.o build/release/libprogram_opts/src/value_store.o

  gcc::Config::new()
        .cpp(true)
        .define("NDEBUG",Some("1"))
        .file("clingo/libprogram_opts/src/application.cpp")
        .file("clingo/libprogram_opts/src/alarm.cpp")
        .file("clingo/libprogram_opts/src/program_options.cpp")
        .file("clingo/libprogram_opts/src/string_convert.cpp")
        .file("clingo/libprogram_opts/src/value_store.cpp")
        .include("clingo/libprogram_opts")
        .compile("libprogram_opts.a");


  gcc::Config::new()
        .cpp(true)
        .define("NDEBUG",Some("1"))
        .file("clingo/libreify/src/program.cc")
        .include("clingo/libreify")
        .include("clingo/liblp")     
        .include("clingo/libgringo")
        .compile("libreify.a");

// ar rc build/release/liblp.a build/release/liblp/src/aspif.o build/release/liblp/src/clingo.o build/release/liblp/src/theory_data.o build/release/liblp/src/smodels.o build/release/liblp/src/convert.o build/release/liblp/src/rule_utils.o build/release/liblp/src/match_basic_types.o build/release/liblp/src/aspif_text.o

  gcc::Config::new()
        .cpp(true)
        .define("NDEBUG",Some("1"))
        .file("clingo/liblp/src/aspif.cpp")
        .file("clingo/liblp/src/clingo.cpp")
        .file("clingo/liblp/src/theory_data.cpp")
        .file("clingo/liblp/src/smodels.cpp")
        .file("clingo/liblp/src/convert.cpp")
        .file("clingo/liblp/src/rule_utils.cpp")
        .file("clingo/liblp/src/match_basic_types.cpp")
        .file("clingo/liblp/src/aspif_text.cpp")
        .include("clingo/liblp")
        .compile("liblp.a");


// ar rc build/release/libclingo.a build/release/libclingo/src/clingocontrol.o

  gcc::Config::new()
        .cpp(true)
        .define("NDEBUG",Some("1"))
        .define("WITH_THREADS",Some("0"))
        .file("clingo/libclingo/src/clingocontrol.cc")
        .include("clingo/libclingo")
        .include("clingo/libgringo")
        .include("clingo/liblp")
        .include("clingo/libclasp")
        .include("clingo/libprogram_opts")
        .compile("libclingo.a");

  gcc::Config::new()
        .cpp(true)
        .define("NDEBUG",Some("1"))
        .define("WITH_THREADS",Some("0"))
        .file("clingo/libclasp/src/parallel_solve.cpp")
        .file("clingo/libclasp/src/solver_types.cpp")
        .file("clingo/libclasp/src/solver_strategies.cpp")
        .file("clingo/libclasp/src/clause.cpp")
        .file("clingo/libclasp/src/logic_program.cpp")
        .file("clingo/libclasp/src/clasp_facade.cpp")
        .file("clingo/libclasp/src/cb_enumerator.cpp")
        .file("clingo/libclasp/src/minimize_constraint.cpp")
        .file("clingo/libclasp/src/clingo.cpp")
        .file("clingo/libclasp/src/shared_context.cpp")
        .file("clingo/libclasp/src/dependency_graph.cpp")
        .file("clingo/libclasp/src/logic_program_types.cpp")
        .file("clingo/libclasp/src/asp_preprocessor.cpp")
        .file("clingo/libclasp/src/clasp_options.cpp")
        .file("clingo/libclasp/src/heuristics.cpp")
        .file("clingo/libclasp/src/timer.cpp")
        .file("clingo/libclasp/src/lookahead.cpp")
        .file("clingo/libclasp/src/constraint.cpp")
        .file("clingo/libclasp/src/unfounded_check.cpp")
        .file("clingo/libclasp/src/solver.cpp")
        .file("clingo/libclasp/src/model_enumerators.cpp")
        .file("clingo/libclasp/src/clasp_output.cpp")
        .file("clingo/libclasp/src/parser.cpp")
        .file("clingo/libclasp/src/solve_algorithms.cpp")
        .file("clingo/libclasp/src/enumerator.cpp")
        .file("clingo/libclasp/src/program_builder.cpp")
        .file("clingo/libclasp/src/satelite.cpp")
        .file("clingo/libclasp/src/statistics.cpp")
        .file("clingo/libclasp/src/weight_constraint.cpp")
        .include("clingo/libclasp")
        .include("clingo/libclingo")
        .include("clingo/liblp")
        .include("clingo/libprogram_opts")
        .compile("libclasp.a");

        

    println!("cargo:rustc-link-lib=clingo");        

    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = bindgen::builder()
        .no_unstable_rust()
        .header("clingo/libgringo/clingo.h")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(out_dir);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


}

