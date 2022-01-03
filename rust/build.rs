use std::env;
use std::fs;
use std::path::PathBuf;

use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct BashCallback;

// rename bash data structures for consistency
impl ParseCallbacks for BashCallback {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        match original_item_name {
            "word_desc" => Some("WordDesc".into()),
            "WORD_DESC" => Some("WordDesc".into()),
            "word_list" => Some("WordList".into()),
            "WORD_LIST" => Some("WordList".into()),
            _ => None,
        }
    }
}

fn main() {
    // generate scallop-specific bindings
    let repo_dir_path = fs::canonicalize(format!("{}/../", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let repo_dir = repo_dir_path.to_str().unwrap();
    let scallop_build_dir = format!("{}/build/src", repo_dir);
    let scallop_src_dir = format!("{}/src", repo_dir);
    let scallop_header = format!("{}/scallop.h", scallop_src_dir);
    println!("cargo:rustc-link-search=native={}", scallop_build_dir);
    println!("cargo:rustc-link-lib=dylib=scallop");
    println!("cargo:rerun-if-changed={}", scallop_header);

    // https://github.com/rust-lang/cargo/issues/4895
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", scallop_build_dir);

    let bindings = bindgen::Builder::default()
        // header to generate bindings for
        .header(scallop_header)
        // invalidate built crate whenever any included header file changes
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("scallop-bindings.rs"))
        .expect("Couldn't write bindings!");

    // generate bash-specific bindings
    println!("cargo:rerun-if-changed=bash-wrapper.h");
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", repo_dir))
        .clang_arg(format!("-I{}/bash", repo_dir))
        .clang_arg(format!("-I{}/bash/include", repo_dir))
        .clang_arg(format!("-I{}/bash/builtins", repo_dir))
        .header("bash-wrapper.h")
        // command.h
        .allowlist_type("word_desc")
        .allowlist_type("word_list")
        // execute_command.h
        .allowlist_var("this_command_name")
        // variables.h
        .allowlist_function("get_string_value")
        // invalidate built crate whenever any included header file changes
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // mangle type names to expected values
        .parse_callbacks(Box::new(BashCallback))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bash-bindings.rs"))
        .expect("Couldn't write bindings!");
}