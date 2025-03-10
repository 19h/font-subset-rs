#![allow(non_snake_case)]

extern crate cpp_build;
extern crate cc;

use cc::Build;
use cpp_build::Config;
use std::env::var;

fn main() {
    compile_cpp_glue_code();
    compile_woff2_library();
}

fn compile_cpp_glue_code() {
    Config::new()
        .include("lib/woff2/include")
        .include("lib/woff2/src")
        .build("src/lib.rs");
}

fn compile_woff2_library() {
    let brotli_include_path = var("DEP_BROTLI_INCLUDE").unwrap();
    
    // e.g., x86_64-apple-darwin15
    let target = var("TARGET").unwrap();
    let compiling_for_macos = target.contains("-darwin");
    
    let mut builder = Build::new();
    builder
        .cpp(true)
        .shared_flag(false)
        .static_flag(true)
        .warnings(false)
        .flag("-fno-omit-frame-pointer")
        .flag("-no-canonical-prefixes")
        .flag("-std=c++11")
        .define("__STDC_FORMAT_MACROS", None);
    
    if compiling_for_macos {
        builder.define("OS_MACOSX", None);
    } else {
        builder.flag("-fno-tree-vrp");
    }
    
    builder
        .include(brotli_include_path)
        .include("lib/woff2/include")
        .file("lib/woff2/src/font.cc")
        .file("lib/woff2/src/glyph.cc")
        .file("lib/woff2/src/normalize.cc")
        .file("lib/woff2/src/table_tags.cc")
        .file("lib/woff2/src/transform.cc")
        .file("lib/woff2/src/woff2_dec.cc")
        .file("lib/woff2/src/woff2_enc.cc")
        .file("lib/woff2/src/woff2_common.cc")
        .file("lib/woff2/src/woff2_out.cc")
        .file("lib/woff2/src/variable_length.cc")
        .compile("woff2");
}
