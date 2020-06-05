extern crate pkg_config;

fn main() {
    if cfg!(feature = "dynamic_linking") {
        match pkg_config::Config::new()
            .atleast_version("5.5.0")
            .probe("clingo")
        {
            Ok(_lib) => {
                println!("cargo:rustc-link-lib=dylib=clingo");
            }
            Err(e) => {
                println!("\nError: {}", e);
                panic!(e);
            }
        }
    } else {
        // update clingo submodule
        // git submodule update --init --recursive

        // create bindings
        // let bindings = bindgen::Builder::default()
        //     .header("clingo/libclingo/clingo.h")
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
        //     .size_t_is_usize(true)
        //     .generate()
        //     .expect("Unable to generate bindings");

        // // Write the bindings to the bindings.rs file.
        // bindings
        //     .write_to_file("bindings.rs")
        //     .expect("Couldn't write bindings!");

        // build clingo for static linking

        use cmake::Config;
        let dst = Config::new("clingo")
            // .very_verbose(true)
            .profile("release")
            .define("CLINGO_BUILD_LIBRARY", "ON")
            .define("CLINGO_NO_VISIBILITY", "ON")
            .define("CLINGO_BUILD_APPS", "OFF")
            .define("CLINGO_BUILD_STATIC", "ON")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .build_target("libclingo")
            .build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build/lib").display()
        );

        println!("cargo:rustc-link-lib=static=reify");
        println!("cargo:rustc-link-lib=static=potassco");
        println!("cargo:rustc-link-lib=static=clasp");
        println!("cargo:rustc-link-lib=static=gringo");
        println!("cargo:rustc-link-lib=static=clingo");
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }
    //     println!("cargo:rustc-link-lib=python3.6m");
    //     -DWITH_PYTHON=1 -I/usr/include/python3.6m
}
