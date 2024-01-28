#![deny(warnings)]

extern crate cmake;

use std::env;
use std::process::{Command, exit};

fn main() {

    let mut meson_cmd = Command::new("meson");
    let build_dir = format!("{}", std::env::var("OUT_DIR").unwrap());
    let src_dir = "src/ots";
    meson_cmd.arg("setup");
    meson_cmd.arg("--force-fallback-for=brotli,libwoff2dec,liblz4,libbrotlidec");

    meson_cmd.arg(src_dir);
    meson_cmd.arg(&build_dir);

    let output = meson_cmd.output().expect("Failed to execute `meson`");
    if !output.status.success() {
        eprintln!("`meson` exited with an error:");
        eprintln!("{output:?}");
        exit(1);
    }

    let mut ninja_cmd = Command::new("ninja");
    ninja_cmd.arg("-C").arg(&build_dir);
    let output = ninja_cmd.output().expect("Failed to execute `ninja`");
    if !output.status.success() {
        eprintln!("`ninja` exited with an error:");
        eprintln!("{output:?}");
        exit(1);
    }

    // Build ots-glue
    let dst = cmake::Config::new("src").build();

    // todo: detect version number for easier updates.
    let woff2_dir = format!("{build_dir}/subprojects/woff2-1.0.2"); //
    let brotli_dir = format!("{build_dir}/subprojects/brotli-1.0.9"); //
    // let subprojects_dir = PathBuf::from(format!("{build_dir}/subprojects/"));
    // if ! subprojects_dir.exists() {
    //     eprintln!()
    // }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}", build_dir);
    println!("cargo:rustc-link-lib=static=ots");
    println!("cargo:rustc-link-search=native={}", woff2_dir);
    println!("cargo:rustc-link-lib=static=woff2_decoder");
    println!("cargo:rustc-link-lib=static=woff2_common");
    // println!("cargo:rustc-link-lib=static=woff2_encoder");
    println!("cargo:rustc-link-search=native={}", brotli_dir);
    println!("cargo:rustc-link-lib=static=brotli_decoder");
    println!("cargo:rustc-link-lib=static=brotli_common");
    // println!("cargo:rustc-link-lib=static=brotli_encoder");
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else if !target.contains("msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
