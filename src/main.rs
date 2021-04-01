use std::env;
use std::path::Path;
use std::process;
use std::process::{Command, Stdio};

struct Alternative<'a> {
    test_file: &'a str,
    command: &'a str,
    default: &'a Vec<String>,
}

fn main() {
    let alternatives = [
        // Make goes first, so you can have a makefile that calls
        // another type of build
        Alternative {
            test_file: "Makefile",
            command: "make",
            default: &vec!["all".to_string()],
        },
        Alternative {
            test_file: "gradlew",
            command: "./gradlew",
            default: &vec!["build".to_string()],
        },
        Alternative {
            test_file: "Cargo.toml",
            command: "cargo",
            default: &vec!["build".to_string()],
        },
        Alternative {
            test_file: "mvnw",
            command: "./mvnw",
            default: &vec!["install".to_string()],
        },
        Alternative {
            test_file: "pom.xml",
            command: "mvn",
            default: &vec!["install".to_string()],
        },
        Alternative {
            test_file: "build.sbt",
            command: "sbt",
            default: &vec!["publishLocal".to_string()],
        },
        Alternative {
            test_file: "build.gradle",
            command: "gradle",
            default: &vec!["build".to_string()],
        },
        Alternative {
            test_file: "package.json",
            command: "npm",
            default: &vec!["install".to_string()],
        },
    ];

    let found = alternatives
        .iter()
        .find(|alt| Path::new(alt.test_file).exists());

    match found {
        Some(alt) => run_command(alt),
        None => {
            eprintln!("Unknown build directory");
            process::exit(1);
        }
    }
}

fn run_command(alt: &Alternative) {
    let actual_args;

    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        actual_args = &args[1..]
    } else {
        actual_args = &alt.default[..]
    }

    println!("Running {} {}", alt.command, actual_args.join(" "));

    let mut process = Command::new(alt.command)
        .args(actual_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start external program");

    process.wait().expect("Error from build process");
}
