extern crate pkg_config;
extern crate gcc;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

macro_rules! errln {
    ($($args:tt)*) => {
        let _ = writeln!(&mut ::std::io::stderr(), $($args)* );
    }
}

fn compute_place_holders(out_dir: &str, inc_dir: &str) {
    let out_rs = Path::new(out_dir).join("placeholders.rs");
    let out_exe = Path::new(out_dir).join("compute_placeholders.exe");

    let mut config = gcc::Config::new();
    config.include(inc_dir);

    let mut command = config.get_compiler().to_command();
    command.arg("compute_placeholders.c");
    if env::var("TARGET").expect("cannot get TARGET") == "msvc" {
        command.arg("/OUT");
    } else {
        command.arg("-o");
    }
    command.arg(&out_exe)
           .stdout(Stdio::null())
           .stderr(Stdio::null());

    errln!("{:?}", command);
    if command.status().is_ok() {
        let mut run_cmd = Command::new(&out_exe);
        errln!("{:?}", run_cmd);
        let output = run_cmd.output().expect("cannot run compute_placeholders.exe");
        let mut f = File::create(&out_rs).unwrap();
        f.write_all(output.stdout.as_slice()).unwrap();
    } else {
        panic!("cannot build compute_placeholders.exe");
    }
}

// fn find_library() {
//     let library = pkg_config::Config::new()
//         .atleast_version("3.0.4")
//         .probe("capstone");
//     if let Ok(library) = library {
//         
//     }
// }

fn main() {

    // Check if capstone is already cloned
    if !Path::new("capstone/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"]).status();
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    compute_place_holders(&out_dir, "capstone/include");

    if !Path::new("capstone/libcapstone.a").exists() {
        build_capstone();
    }
    
    println!("cargo:rustc-link-search=native=capstone");
    println!("cargo:rustc-link-lib=static=capstone")
}

fn build_capstone() {

    let target = env::var("TARGET").unwrap();

    let host = env::var("HOST").unwrap();
    let host_windows = host.contains("windows");

    // let windows = target.contains("windows");
    let msvc = target.contains("msvc");

    if msvc && !host_windows {
        panic!("cannot cross compile to msvc");
    }
    
    if msvc {
        panic!("msvc build not supported yet");
    }

    let mut cmd = Command::new("bash");
    cmd.current_dir("capstone")
        .arg("make.sh")
        .env("CAPSTONE_SHARED", "no")
        .env("CAPSTONE_STATIC", "yes");

    if host == target {
        // do nothing
    } else if target == "i686-pc-windows-gnu" {
        cmd.arg("cross-win32");
    } else if target == "x86_64-pc-windows-gnu" {
        cmd.arg("cross-win64");
    } else if host.starts_with("x86_64-") && target.starts_with("i686-") && host[6..] == target[4..] {
        // not perfect but hey
        cmd.arg("nix32");
    } else {
        cmd.env("CROSS", target+"-");
    }


    let res = cmd.status();
    if let Ok(res) = res {
        if !res.success() {
            panic!("cannot compile capstone");
        }
    } else {
        panic!("cannot start capstone build");
    }


    // let include_paths =
    //     if let Ok(library) = pkg_config::probe_library("capstone") {
    //         library.include_paths
    //     } else {
    //         errln!("cannot find capstone with pkg_config");
    //         Vec::new()
    //     };
    //
    // let out_dir = env::var("OUT_DIR").unwrap();
    // compute_place_holders(&out_dir, &include_paths);
}
