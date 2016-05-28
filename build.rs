use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("placeholders.rs");

    let mut compile_command: Command = Command::new("gcc");
    compile_command
        .arg("compute_placeholders.c")
        .arg("-o")
        .arg("compute_placeholders.exe")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if compile_command.status().is_ok() {
        let output = Command::new("./compute_placeholders.exe").output().expect("cannot run compute_placeholders.exe");
        let mut f = File::create(&dest_path).unwrap();
        f.write_all(output.stdout.as_slice()).unwrap();
    }

}
