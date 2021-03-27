use std::path::Path;
use std::process;
use std::process::{Command, Stdio};

fn main() {
    let command;
    let default;
    if Path::new("./gradlew").exists() {
        command = "./gradlew";
        default = "build";
    } else if Path::new("./Cargo.toml").exists() {
        command = "cargo";
        default = "build";
    } else {
        eprintln!("Unknown build directory");
        process::exit(1);
    }

    println!("Running {} {}", command, default);

    let mut process = Command::new(command)
        .args(default.split(' '))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start external program");

    process.wait().expect("Error from build process");
}
