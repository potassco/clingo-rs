use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if cfg!(feature = "dynamic_linking") {
        println!("cargo:rustc-link-lib=dylib=clingo");
    } else {
        // build clingo for static linking

        let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR was not set"));
        let clingo_dir = out_dir.join("clingo");

        if !clingo_dir.exists() {
            Command::new("git")
                .args(&["clone", "https://github.com/potassco/clingo.git"])
                .current_dir(out_dir.to_str().unwrap())
                .status()
                .unwrap();

            Command::new("git")
                .args(&["checkout", "wip"])
                .current_dir(clingo_dir.to_str().unwrap())
                .status()
                .unwrap();

            Command::new("git")
                .args(&["submodule", "update", "--init", "--recursive"])
                .current_dir(clingo_dir.to_str().unwrap())
                .status()
                .unwrap();
        }

        if !Path::new("bindings.rs").exists() {
            let bindings = bindgen::Builder::default()
                .header(clingo_dir.join("libclingo/clingo.h").to_str().unwrap())
                .no_copy("clingo_solve_control")
                .no_copy("clingo_model")
                .no_copy("clingo_solve_handle")
                .no_copy("clingo_program_builder")
                .no_copy("clingo_control")
                .no_copy("clingo_options")
                .no_copy("clingo_symbolic_atoms")
                .no_copy("clingo_theory_atoms")
                .no_copy("clingo_assignment")
                .no_copy("clingo_propagate_init")
                .no_copy("clingo_propagate_control")
                .no_copy("clingo_backend")
                .no_copy("clingo_configuration")
                .no_copy("clingo_statistic")
                .blacklist_type("max_align_t") // https://github.com/rust-lang/rust-bindgen/issues/550
                .generate()
                .expect("Unable to generate bindings");
        
            // Write the bindings to the bindings.rs file.
            bindings
                .write_to_file("bindings.rs")
                .expect("Couldn't write bindings!");
        }

        // libpotassco
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++14")
            .flag("-O3")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .file(clingo_dir.join("clasp/libpotassco/src/application.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/aspif.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/aspif_text.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/clingo.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/convert.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/match_basic_types.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/program_options.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/rule_utils.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/smodels.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/string_convert.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/theory_data.cpp"))
            .file(clingo_dir.join("clasp/libpotassco/src/value_store.cpp"))
            .include(clingo_dir.join("clasp/libpotassco"))
            .compile("libpotassco.a");

        // libclasp
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .define("WITH_THREADS", Some("0"))
            .file(clingo_dir.join("clasp/src/asp_preprocessor.cpp"))
            .file(clingo_dir.join("clasp/src/cb_enumerator.cpp"))
            .file(clingo_dir.join("clasp/src/clasp_facade.cpp"))
            .file(clingo_dir.join("clasp/src/clasp_options.cpp"))
            .file(clingo_dir.join("clasp/src/clasp_output.cpp"))
            .file(clingo_dir.join("clasp/src/clause.cpp"))
            .file(clingo_dir.join("clasp/src/clingo.cpp"))
            .file(clingo_dir.join("clasp/src/constraint.cpp"))
            .file(clingo_dir.join("clasp/src/dependency_graph.cpp"))
            .file(clingo_dir.join("clasp/src/enumerator.cpp"))
            .file(clingo_dir.join("clasp/src/heuristics.cpp"))
            .file(clingo_dir.join("clasp/src/logic_program.cpp"))
            .file(clingo_dir.join("clasp/src/logic_program_types.cpp"))
            .file(clingo_dir.join("clasp/src/lookahead.cpp"))
            .file(clingo_dir.join("clasp/src/minimize_constraint.cpp"))
            .file(clingo_dir.join("clasp/src/model_enumerators.cpp"))
            .file(clingo_dir.join("clasp/src/parser.cpp"))
            .file(clingo_dir.join("clasp/src/program_builder.cpp"))
            .file(clingo_dir.join("clasp/src/satelite.cpp"))
            .file(clingo_dir.join("clasp/src/shared_context.cpp"))
            .file(clingo_dir.join("clasp/src/solve_algorithms.cpp"))
            .file(clingo_dir.join("clasp/src/solver.cpp"))
            .file(clingo_dir.join("clasp/src/solver_strategies.cpp"))
            .file(clingo_dir.join("clasp/src/solver_types.cpp"))
            .file(clingo_dir.join("clasp/src/statistics.cpp"))
            .file(clingo_dir.join("clasp/src/timer.cpp"))
            .file(clingo_dir.join("clasp/src/unfounded_check.cpp"))
            .file(clingo_dir.join("clasp/src/weight_constraint.cpp"))
            .file(clingo_dir.join("clasp/src/parallel_solve.cpp"))
            .include(clingo_dir.join("clasp"))
            .include("generated")
            .include(clingo_dir.join("clasp/libpotassco"))
            .compile("libclasp.a");

        // libgringo
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .file(clingo_dir.join("libgringo/src/backend.cc"))
            .file(clingo_dir.join("libgringo/src/primes.cc"))
            .file(clingo_dir.join("libgringo/src/symbol.cc"))
            .file(clingo_dir.join("libgringo/src/term.cc"))
            .file(clingo_dir.join("libgringo/src/terms.cc"))
            .file(clingo_dir.join("libgringo/src/ground/instantiation.cc"))
            .file(clingo_dir.join("libgringo/src/ground/literals.cc"))
            .file(clingo_dir.join("libgringo/src/ground/program.cc"))
            .file(clingo_dir.join("libgringo/src/ground/statements.cc"))
            .file(clingo_dir.join("libgringo/src/input/aggregate.cc"))
            .file(clingo_dir.join("libgringo/src/input/aggregates.cc"))
            .file(clingo_dir.join("libgringo/src/input/groundtermparser.cc"))
            .file(clingo_dir.join("libgringo/src/input/literal.cc"))
            .file(clingo_dir.join("libgringo/src/input/literals.cc"))
            .file(clingo_dir.join("libgringo/src/input/nongroundparser.cc"))
            .file(clingo_dir.join("libgringo/src/input/program.cc"))
            .file(clingo_dir.join("libgringo/src/input/programbuilder.cc"))
            .file(clingo_dir.join("libgringo/src/input/statement.cc"))
            .file(clingo_dir.join("libgringo/src/input/theory.cc"))
            .file("generated/input/groundtermgrammar/grammar.cc")
            .file("generated/input/nongroundgrammar/grammar.cc")
            .file(clingo_dir.join("libgringo/src/output/aggregates.cc"))
            .file(clingo_dir.join("libgringo/src/output/literal.cc"))
            .file(clingo_dir.join("libgringo/src/output/literals.cc"))
            .file(clingo_dir.join("libgringo/src/output/output.cc"))
            .file(clingo_dir.join("libgringo/src/output/statement.cc"))
            .file(clingo_dir.join("libgringo/src/output/statements.cc"))
            .file(clingo_dir.join("libgringo/src/output/theory.cc"))
            .include(clingo_dir.join("libgringo"))
            .include("generated")
            .include(clingo_dir.join("clasp/libpotassco"))
            .include(clingo_dir.join("libreify"))
            .compile("libgringo.a");

        // libclingo
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .define("WITH_THREADS", Some("0"))
            .file(clingo_dir.join("libclingo/src/ast.cc"))
            .file(clingo_dir.join("libclingo/src/clingo_app.cc"))
            .file(clingo_dir.join("libclingo/src/clingocontrol.cc"))
            .file(clingo_dir.join("libclingo/src/control.cc"))
            .file(clingo_dir.join("libclingo/src/gringo_app.cc"))
            .file(clingo_dir.join("libclingo/src/incmode.cc"))
            .file(clingo_dir.join("libclingo/src/scripts.cc"))
            .file(clingo_dir.join("clasp/app/clasp_app.cpp"))
            .include(clingo_dir.join("libclingo"))
            .include(clingo_dir.join("libgringo"))
            .include(clingo_dir.join("clasp/libpotassco"))
            .include(clingo_dir.join("clasp"))
            .include(clingo_dir.join("clasp/app"))
            .include("generated")
            .compile("libclingo.a");

        // libreify
        cc::Build::new()
            .cpp(true)
            .flag("-O3")
            .flag("-std=c++14")
            .warnings(false)
            .define("NDEBUG", Some("1"))
            .file(clingo_dir.join("libreify/src/program.cc"))
            .include(clingo_dir.join("libreify"))
            .include(clingo_dir.join("libgringo"))
            .include(clingo_dir.join("clasp/libpotassco"))
            .compile("libreify.a");

        println!("cargo:rustc-link-lib=static=potassco");
        println!("cargo:rustc-link-lib=static=clasp");
        println!("cargo:rustc-link-lib=static=gringo");
        println!("cargo:rustc-link-lib=static=clingo");
    }
    //     println!("cargo:rustc-link-lib=python3.6m");
    //     -DWITH_PYTHON=1 -I/usr/include/python3.6m
}
