use std::process::Command;

fn main() {
    let r_include = Command::new("R")
        .args(["CMD", "config", "--cppflags"])
        .output()
        .unwrap()
        .stdout;
    let r_include = String::from_utf8_lossy(&r_include);
    dbg!(&r_include);

    let bindings = bindgen::builder()
        .clang_args(r_include.split_ascii_whitespace())
        .allowlist_file("wk/inst/include/wk-v1.h")
        .allowlist_file("wk/inst/include/wk-v1-impl.c")
        .allowlist_file("wk/inst/include/experimental/wk-v1-filter-cpp11.hpp")
        .allowlist_file("wk/inst/include/experimental/wk-v1-handler-cpp11.hpp")
        .allowlist_file("wk/inst/include/experimental/wk-v1-reader-cpp11.hpp");

    let base = bindings
        .clone()
        .header("wk/inst/include/wk-v1.h")
        .generate()
        .unwrap();
    base.write_to_file("src/bindings_wk.rs").unwrap();
}
