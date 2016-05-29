extern crate pkg_config;
extern crate gcc;

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::{Path,PathBuf};
use std::process::{Command, Stdio};

macro_rules! errln {
    ($($args:tt)*) => {
        let _ = writeln!(&mut ::std::io::stderr(), $($args)* );
    }
}

fn compute_place_holders(out_dir: &str, includes: &[PathBuf]) {
    let out_rs = Path::new(out_dir).join("placeholders.rs");
    let out_exe = Path::new(out_dir).join("compute_placeholders.exe");

    let mut config = gcc::Config::new();
    for inc in includes {
        config.include(inc);
    }

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


fn main() {

    let include_paths =
        if let Ok(library) = pkg_config::probe_library("capstone") {
            library.include_paths
        } else {
            errln!("cannot find capstone with pkg_config");
            Vec::new()
        };

    let out_dir = env::var("OUT_DIR").unwrap();
    compute_place_holders(&out_dir, &include_paths);
}
