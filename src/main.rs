use std::env;
use std::path::Path;
use std::process;
use std::process::{Command, Stdio};

fn main() {
    let command;
    let default;
    let actual_args;
    if Path::new("./gradlew").exists() {
        command = "./gradlew";
        default = [String::from("build")];
    } else if Path::new("./Cargo.toml").exists() {
        command = "cargo";
        default = [String::from("build")];
    } else {
        eprintln!("Unknown build directory");
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        actual_args = &args[1..]
    } else {
        actual_args = &default
    }

    println!("Running {} {}", command, actual_args.join(" "));

    let mut process = Command::new(command)
        .args(actual_args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start external program");

    process.wait().expect("Error from build process");
}
