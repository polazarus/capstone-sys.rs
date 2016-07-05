extern crate gcc;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

macro_rules! errln {
    ($($args:tt)*) => {
        let _ = writeln!(&mut ::std::io::stderr(), $($args)* );
    }
}

fn main() {
    let target = env::var("TARGET").expect("cannot get TARGET");
    let host = env::var("HOST").expect("cannot get HOST");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("cannot get OUT_DIR"));
    let msvc = target.contains("msvc");

    // Check if capstone is already cloned
    if !Path::new("capstone/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"]).status();
    }

    if !target.starts_with("x86_64-") && !target.starts_with("i686") {
        compute_place_holders(&out_dir, "capstone/include", msvc);
    }

    let libcapstone = out_dir.join("libcapstone.a");
    if !libcapstone.exists() {
        build_capstone(&out_dir, &target, &host, msvc);
    }
    if !libcapstone.exists() {
        panic!("cannot find libcapstone")
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=capstone")
}

fn compute_place_holders(out_dir: &Path, inc_dir: &str, msvc: bool) {
    let out_rs = out_dir.join("placeholders.rs");
    let out_exe = out_dir.join("compute_placeholders.exe");

    let mut config = gcc::Config::new();
    config.include(inc_dir);

    let mut command = config.get_compiler().to_command();
    command.arg("compute_placeholders.c");
    if msvc {
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

fn build_capstone(out_dir: &Path, target: &str, host: &str, msvc: bool) {

    let host_windows = host.contains("windows");

    if msvc && !host_windows {
        panic!("cannot cross compile to msvc");
    }

    if msvc {
        panic!("msvc build not supported yet");
    }

    let mut cmd = Command::new("make");
    cmd.current_dir("capstone")
        // .env("CFLAGS", "-flto")
        .env("BUILDDIR", out_dir)
        .env("CAPSTONE_BUILD_CORE_ONLY", "yes")
        .env("CAPSTONE_SHARED", "no")
        .env("CAPSTONE_STATIC", "yes");

    if host == target {
        // do nothing
    } else if target == "i686-pc-windows-gnu" {
        cmd.arg("cross-win32");
    } else if target == "x86_64-pc-windows-gnu" {
        cmd.arg("cross-win64");
    } else if host.starts_with("x86_64-") && target.starts_with("i686-") && host[7..] == target[5..] {
        // not perfect but hey
        cmd.arg("nix32");
    } else {
        cmd.env("CROSS", target.to_string() + "-");
    }

    errln!("build capstone: {:?}", cmd);

    let res = cmd.status();
    if let Ok(res) = res {
        if !res.success() {
            panic!("cannot compile capstone");
        }
    } else {
        panic!("cannot start capstone build");
    }

}
