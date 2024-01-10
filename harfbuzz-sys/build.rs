// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn build_harfbuzz() {
    use std::env;
    use std::path::PathBuf;

    let target = env::var("TARGET").unwrap();

    // for the Rust esp32 IDF configuration, we need to specify the target compilers
    if target == "xtensa-esp32-espidf" {
        env::set_var("CXX", "xtensa-esp32-elf-g++");
        env::set_var("CC", "xtensa-esp32-elf-gcc");
        env::set_var("CFLAGS", "-mlongcalls");
        env::set_var("CXXFLAGS", "-mlongcalls");
    } else if target == "xtensa-esp32s3-espidf" {
        env::set_var("CXX", "xtensa-esp32s3-elf-g++");
        env::set_var("CC", "xtensa-esp32s3-elf-gcc");
        env::set_var("AR", "xtensa-esp32s3-elf-ar");
        env::set_var("CFLAGS", "-mlongcalls -ffunction-sections -fdata-sections");
        env::set_var(
            "CXXFLAGS",
            "-mlongcalls -Wno-frame-address -ffunction-sections -fdata-sections",
        );
    }

    let mut cfg = cc::Build::new();
    if !target.contains("xtensa") && cfg!(feature = "freetype") {
        pkg_config::probe_library("freetype2").unwrap();
        cfg.define("HAVE_FREETYPE", "1");
        cfg.flag("-I/usr/include/freetype2");
    }
    cfg.cpp(true)
        .flag_if_supported("-std=c++11") // for unix
        .warnings(false)
        .file("harfbuzz/src/harfbuzz.cc");

    if !target.contains("windows") {
        cfg.define("HAVE_PTHREAD", "1");
    }

    if target.contains("apple") && cfg!(feature = "coretext") {
        cfg.define("HAVE_CORETEXT", "1");
    }

    if target.contains("windows") && cfg!(feature = "directwrite") {
        cfg.define("HAVE_DIRECTWRITE", "1");
    }

    if target.contains("windows-gnu") {
        cfg.flag("-Wa,-mbig-obj");
    }

    cfg.compile("embedded_harfbuzz");

    let out_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!(
        "cargo:include={}",
        out_dir.join("harfbuzz").join("src").display()
    );
}

fn main() {
    if cfg!(feature = "bundled") {
        build_harfbuzz();
    } else {
        // Use the pre-installed harfbuzz.
        pkg_config::probe_library("harfbuzz").unwrap();
    }
}
