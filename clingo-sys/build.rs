extern crate gcc;

use std::process::Command;


fn main() {

    Command::new("git")
        .args(&["clone", "https://github.com/potassco/clingo.git"])
        .status()
        .unwrap();

    Command::new("git")
        .args(&["checkout", "tags/v5.1.0"])
        .current_dir("./clingo")
        .status()
        .unwrap();

    gcc::Config::new()
        .cpp(true)
        .define("NDEBUG", Some("1"))
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
        .file("generated/input/nongroundgrammar/grammar.cc")
        .file("generated/input/groundtermgrammar/grammar.cc")
        .file("clingo/libgringo/src/input/nongroundparser.cc")
        .file("clingo/libgringo/src/input/aggregate.cc")
        .file("clingo/libgringo/src/input/aggregates.cc")
        .file("clingo/libgringo/src/input/program.cc")
        .file("clingo/libgringo/src/ground/program.cc")
        .file("clingo/libgringo/src/input/programbuilder.cc")
        .file("clingo/libgringo/src/input/literals.cc")
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
        .include("generated")
        .include("clingo/liblp")
        .include("clingo/libreify")
        .compile("libgringo.a");

    gcc::Config::new()
        .cpp(true)
        .define("NDEBUG", Some("1"))
        .file("clingo/libprogram_opts/src/application.cpp")
        .file("clingo/libprogram_opts/src/alarm.cpp")
        .file("clingo/libprogram_opts/src/program_options.cpp")
        .file("clingo/libprogram_opts/src/string_convert.cpp")
        .file("clingo/libprogram_opts/src/value_store.cpp")
        .include("clingo/libprogram_opts")
        .compile("libprogram_opts.a");

    gcc::Config::new()
        .cpp(true)
        .define("NDEBUG", Some("1"))
        .file("clingo/libreify/src/program.cc")
        .include("clingo/libreify")
        .include("clingo/liblp")
        .include("clingo/libgringo")
        .compile("libreify.a");

    gcc::Config::new()
        .cpp(true)
        .define("NDEBUG", Some("1"))
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

    gcc::Config::new()
        .cpp(true)
        .define("NDEBUG", Some("1"))
        .define("WITH_THREADS", Some("0"))
        .file("clingo/libclingo/src/clingocontrol.cc")
        .include("clingo/libclingo")
        .include("clingo/libgringo")
        .include("clingo/liblp")
        .include("clingo/libclasp")
        .include("clingo/libprogram_opts")
        .compile("libclingo.a");

    gcc::Config::new()
        .cpp(true)
        .define("NDEBUG", Some("1"))
        .define("WITH_THREADS", Some("0"))
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

    println!("cargo:rustc-link-lib=static=clingo");
    println!("cargo:rustc-link-lib=static=clasp");
    println!("cargo:rustc-link-lib=static=program_opts");
    println!("cargo:rustc-link-lib=static=lp");
    println!("cargo:rustc-link-lib=static=reify");
    println!("cargo:rustc-link-lib=static=gringo");
    
//     println!("cargo:rustc-link-lib=python3.6m");
//     -DWITH_PYTHON=1 -I/usr/include/python3.6m
    
}
