use std::env;
use std::path::PathBuf;
use std::process::Command;
extern crate pkg_config;

fn main() {
    if cfg!(feature = "dynamic_linking") {

        match pkg_config::Config::new().atleast_version("5.4.0").probe("clingo"){
            Ok(_lib) => {println!("cargo:rustc-link-lib=dylib=clingo");},
            Err(e) => {
                println!("\nError: {}", e);
                panic!(e);
            }
        }
        
    } else {

        // download clingo / clasp from github
        
        // git clone https://github.com/potassco/clingo.git
        // git checkout v5.4.0
        // git submodule update --init --recursive


        // create bindings

        // if !Path::new("bindings.rs").exists() {
            // let bindings = bindgen::Builder::default()
            //     .header("clingo/libclingo/clingo.h").to_str().unwrap())
            //     .no_copy("clingo_solve_control")
            //     .no_copy("clingo_model")
            //     .no_copy("clingo_solve_handle")
            //     .no_copy("clingo_program_builder")
            //     .no_copy("clingo_control")
            //     .no_copy("clingo_options")
            //     .no_copy("clingo_symbolic_atoms")
            //     .no_copy("clingo_theory_atoms")
            //     .no_copy("clingo_assignment")
            //     .no_copy("clingo_propagate_init")
            //     .no_copy("clingo_propagate_control")
            //     .no_copy("clingo_backend")
            //     .no_copy("clingo_configuration")
            //     .no_copy("clingo_statistic")
            //     // .no_copy("clingo_ast_term")
            //     // .no_copy("clingo_ast_function")
            //     // .no_copy("clingo_ast_pool")
            //     // .no_copy("clingo_ast_csp_product_term_t")
            //     .blacklist_type("max_align_t") // https://github.com/rust-lang/rust-bindgen/issues/550
            //     .generate()
            //     .expect("Unable to generate bindings");
        
            // // Write the bindings to the bindings.rs file.
            // bindings
            //     .write_to_file("bindings.rs")
            //     .expect("Couldn't write bindings!");
        // }

        // build clingo for static linking

        // libpotassco
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++14")
            .flag("-O3")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .file("clingo/clasp/libpotassco/src/application.cpp")
            .file("clasp/libpotassco/src/aspif.cpp")
            .file("clingo/clasp/libpotassco/src/aspif_text.cpp")
            .file("clingo/clasp/libpotassco/src/clingo.cpp")
            .file("clingo/clasp/libpotassco/src/convert.cpp")
            .file("clingo/clasp/libpotassco/src/match_basic_types.cpp")
            .file("clingo/clasp/libpotassco/src/program_options.cpp")
            .file("clingo/clasp/libpotassco/src/rule_utils.cpp")
            .file("clingo/clasp/libpotassco/src/smodels.cpp")
            .file("clingo/clasp/libpotassco/src/string_convert.cpp")
            .file("clingo/clasp/libpotassco/src/theory_data.cpp")
            .file("clingo/clasp/libpotassco/src/value_store.cpp")
            .include("clingo/clasp/libpotassco"))
            .compile("libpotassco.a");

        // libclasp
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .define("WITH_THREADS", Some("0"))
            .file("clingo/clasp/src/asp_preprocessor.cpp")
            .file("clingo/clasp/src/cb_enumerator.cpp")
            .file("clingo/clasp/src/clasp_facade.cpp")
            .file("clingo/clasp/src/clasp_options.cpp")
            .file("clingo/clasp/src/clasp_output.cpp")
            .file("clingo/clasp/src/clause.cpp")
            .file("clingo/clasp/src/clingo.cpp")
            .file("clingo/clasp/src/constraint.cpp")
            .file("clingo/clasp/src/dependency_graph.cpp")
            .file("clingo/clasp/src/enumerator.cpp")
            .file("clingo/clasp/src/heuristics.cpp")
            .file("clingo/clasp/src/logic_program.cpp")
            .file("clingo/clasp/src/logic_program_types.cpp")
            .file("clingo/clasp/src/lookahead.cpp")
            .file("clingo/clasp/src/minimize_constraint.cpp")
            .file("clingo/clasp/src/model_enumerators.cpp")
            .file("clingo/clasp/src/parser.cpp")
            .file("clingo/clasp/src/program_builder.cpp")
            .file("clingo/clasp/src/satelite.cpp")
            .file("clingo/clasp/src/shared_context.cpp")
            .file("clingo/clasp/src/solve_algorithms.cpp")
            .file("clingo/clasp/src/solver.cpp")
            .file("clingo/clasp/src/solver_strategies.cpp")
            .file("clingo/clasp/src/solver_types.cpp")
            .file("clingo/clasp/src/statistics.cpp")
            .file("clingo/clasp/src/timer.cpp")
            .file("clingo/clasp/src/unfounded_check.cpp")
            .file("clingo/clasp/src/weight_constraint.cpp")
            .file("clingo/clasp/src/parallel_solve.cpp")
            .include("clingo/clasp"))
            .include("generated")
            .include("clingo/clasp/libpotassco"))
            .compile("libclasp.a");

        // libgringo
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .file("clingo/libgringo/src/backend.cc"))
            .file("clingo/libgringo/src/primes.cc"))
            .file("clingo/libgringo/src/symbol.cc"))
            .file("clingo/libgringo/src/term.cc"))
            .file("clingo/libgringo/src/terms.cc"))
            .file("clingo/libgringo/src/ground/instantiation.cc"))
            .file("clingo/libgringo/src/ground/literals.cc"))
            .file("clingo/libgringo/src/ground/program.cc"))
            .file("clingo/libgringo/src/ground/statements.cc"))
            .file("clingo/libgringo/src/input/aggregate.cc"))
            .file("clingo/libgringo/src/input/aggregates.cc"))
            .file("clingo/libgringo/src/input/groundtermparser.cc"))
            .file("clingo/libgringo/src/input/literal.cc"))
            .file("clingo/libgringo/src/input/literals.cc"))
            .file("clingo/libgringo/src/input/nongroundparser.cc"))
            .file("clingo/libgringo/src/input/program.cc"))
            .file("clingo/libgringo/src/input/programbuilder.cc"))
            .file("clingo/libgringo/src/input/statement.cc"))
            .file("clingo/libgringo/src/input/theory.cc"))
            .file("generated/input/groundtermgrammar/grammar.cc")
            .file("generated/input/nongroundgrammar/grammar.cc")
            .file("clingo/libgringo/src/output/aggregates.cc"))
            .file("clingo/libgringo/src/output/literal.cc"))
            .file("clingo/libgringo/src/output/literals.cc"))
            .file("clingo/libgringo/src/output/output.cc"))
            .file("clingo/libgringo/src/output/statement.cc"))
            .file("clingo/libgringo/src/output/statements.cc"))
            .file("clingo/libgringo/src/output/theory.cc"))
            .include("clingo/libgringo"))
            .include("generated")
            .include("clingo/clasp/libpotassco"))
            .include("clingo/libreify"))
            .compile("libgringo.a");

        // libclingo
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .define("WITH_THREADS", Some("0"))
            .file("clingo/libclingo/src/ast.cc"))
            .file("clingo/libclingo/src/clingo_app.cc"))
            .file("clingo/libclingo/src/clingocontrol.cc"))
            .file("clingo/libclingo/src/control.cc"))
            .file("clingo/libclingo/src/gringo_app.cc"))
            .file("clingo/libclingo/src/incmode.cc"))
            .file("clingo/libclingo/src/scripts.cc"))
            .file("clingo/clasp/app/clasp_app.cpp")
            .include("clingo/libclingo"))
            .include("clingo/libgringo"))
            .include("clingo/clasp/libpotassco"))
            .include("clingo/clasp"))
            .include("clingo/clasp/app"))
            .include("generated")
            .compile("libclingo.a");

        // libreify
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .file("clingo/libreify/src/program.cc"))
            .include("clingo/libreify"))
            .include("clingo/libgringo"))
            .include("clingo/clasp/libpotassco"))
            .compile("libreify.a");

        println!("cargo:rustc-link-lib=static=potassco");
        println!("cargo:rustc-link-lib=static=clasp");
        println!("cargo:rustc-link-lib=static=gringo");
        println!("cargo:rustc-link-lib=static=clingo");
    }
    //     println!("cargo:rustc-link-lib=python3.6m");
    //     -DWITH_PYTHON=1 -I/usr/include/python3.6m
}
