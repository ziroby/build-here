use std::path::Path;
use std::process;

fn main() {
    let command;
    let default;
    if Path::new("./gradlew").exists() {
        command = "./gradlew";
        default = "build";
    } else {
        eprintln!("Unknown build directory");
        process::exit(1);
    }

    println!("Running {} {}", command, default);
}
