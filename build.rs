use std::{path::PathBuf, process::Command};

fn main() {
    // UNCOMMENT if you need the linker...
    // println!("cargo:rustc-link-search=libgcc_mock");
    let cppflags = Command::new("R")
        .args(["CMD", "config", "--cppflags"])
        .output()
        .unwrap()
        .stdout;
    let r_include = String::from_utf8(cppflags).unwrap();
    let r_include = r_include.trim();

    let r_tools_soft = Command::new("R")
        .args(["CMD", "config", "R_TOOLS_SOFT"])
        .output()
        .unwrap()
        .stdout;
    let r_tools_soft = String::from_utf8(r_tools_soft).unwrap();
    let r_tools_soft = r_tools_soft.trim();
    let r_tools_soft = PathBuf::from(r_tools_soft);

    dbg!(&r_include, &r_tools_soft);
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::builder()
        
        // only affected `wk_geometery_type_enum`
        .rustified_enum(".*")

        .sort_semantically(true)
        .clang_args(r_include.split_ascii_whitespace())
        // .detect_include_paths(true)
        .clang_arg(format!("-I{}", r_tools_soft.join("include").display()))
        // ignore some gcc specific attributes are used
        // .clang_arg("-Wno-ignored-attributes")

        // various options.. that might need to be removed.
        // .clang_args("-O2 -Wall -std=gnu2x -mfpmath=sse -msse2 -mstackrealign".split_ascii_whitespace())
        // .clang_arg("-std=gnu2x")
        // .clang_args("-shared -s -static-libgcc".split_ascii_whitespace())
        // .clang_arg("-shared")
        // .clang_arg("-Llibgcc_mock")
        // .clang_arg("-static-libgcc")

        // this ensures only these things are included in the bindgen
        .allowlist_file("wk/inst/include/wk-v1.h")
        .allowlist_file("wk/inst/include/wk-v1-impl.c")
        // .allowlist_file("wk/inst/include/wk/experimental/wk-v1-filter-cpp11.hpp")
        // .allowlist_file("wk/inst/include/wk/experimental/wk-v1-handler-cpp11.hpp")
        // .allowlist_file("wk/inst/include/wk/experimental/wk-v1-reader-cpp11.hpp")
        ;

    bindings
        .clone()
        .header("wk/inst/include/wk-v1.h")
        .generate()
        .unwrap()
        .write_to_file("src/bindings_wk.rs")
        .unwrap();

    // wk\inst\include\wk\experimental\wk-v1-filter-cpp11.hpp
    // this isn't building on msvc
    // bindings
    //     .clone()
    //     .clang_arg("-Icpp11/inst/include")
    //     .clang_arg("-Iwk/inst/include")
    //     .detect_include_paths(true)
    //     .clang_arg("-std=gnu++20")
    //     .clang_arg(format!(
    //         "-I{}/include/boost/compatibility/cpp_c_headers",
    //         r_tools_soft.display()
    //     ))
    //     .header("wk/inst/include/wk/experimental/wk-v1-filter-cpp11.hpp")
    //     .generate()
    //     .unwrap()
    //     .write_to_file("src/bindings_wk_filter.rs")
    //     .unwrap();
}
