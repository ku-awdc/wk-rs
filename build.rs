use std::process::Command;

/// Returns the result of running `R CMD config [flag]`.
///
/// Panics if R isn't available in `PATH`.
///
pub fn run_r_cmd_config(flag: &str) -> String {
    let r_cmd_config_flag = Command::new("R")
        .args(["CMD", "config", flag])
        .output()
        .unwrap()
        .stdout;
    let result = String::from_utf8(r_cmd_config_flag).unwrap();
    let result = result.trim();
    result.to_string()
}

fn main() {
    // UNCOMMENT if you need the linker...
    println!("cargo:rustc-link-search=libgcc_mock");
    let cppflags = run_r_cmd_config("--cppflags");

    #[cfg(feature = "export_bindings")]
    export_bindings(&cppflags);

    let ldflags = run_r_cmd_config("--ldflags");
    dbg!(&ldflags);

    let mut wk_build = cc::Build::new();
    let mut wk_build = wk_build
        .cargo_metadata(true)
        .file("wk/inst/include/wk-v1-impl.c");
    for flag in cppflags
        .split_ascii_whitespace()
        .chain(ldflags.split_ascii_whitespace())
    {
        println!("cargo:rustc-link-arg={}", flag);
        wk_build = wk_build.flag(flag);
    }

    let cxx23flags = run_r_cmd_config("CXX23FLAGS");
    dbg!(&cxx23flags);
    for flag in cxx23flags.split_ascii_whitespace() {
        wk_build = wk_build.flag(flag);
    }

    wk_build
        // -s, --strip-all             Strip all symbols
        .flag("-s")
        .debug(false)
        // warning that isn't useful
        .flag("-Wno-unused-parameter")
        .extra_warnings(true)
        .shared_flag(true)
        .compile("wk");
}

#[cfg(feature = "export_bindings")]
fn export_bindings(cppflags: &String) {
    let r_tools_soft = run_r_cmd_config("R_TOOLS_SOFT");

    dbg!(&cppflags, &r_tools_soft);
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::builder()
        // does nothing
        // .array_pointers_in_arguments(true)

        // .wrap_unsafe_ops(true)
        // only affected `wk_geometery_type_enum_type`
        .merge_extern_blocks(true)

        // only affected `wk_geometery_type_enum`
        .rustified_enum(".*")

        // does nothing
        .enable_function_attribute_detection()

        // .enable_cxx_namespaces()

        .sort_semantically(true)
        .layout_tests(false)
        // .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)

        .wrap_unsafe_ops(true)

        .clang_args(cppflags.split_ascii_whitespace())
        // .detect_include_paths(true)
        .clang_arg(format!("-I{r_tools_soft}/include"))
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
