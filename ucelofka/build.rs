use std::{env, process::Command};

fn call_clean() {
    let args = vec!["build", "--dist", "../webapp/"];

    let status = Command::new("trunk")
        .args(&args)
        .status()
        .expect("Failed to clean using trunk");
    println!("clean process exited with: {}", status);
    if !status.success() {
        panic!("clean process failed");
    }
}

fn call_build() {
    let mut args = vec!["build", "--dist", "../webapp/"];
    let profile = env::var("PROFILE").unwrap();
    if profile == "release" {
        args.push("--release");
    }

    let status = Command::new("trunk")
        .args(&args)
        .status()
        .expect("Failed to build using trunk");
    println!("build process exited with: {}", status);
    if !status.success() {
        panic!("build process failed");
    }
}

fn main() {
    let current = env::current_dir().expect("Failed to get current directory.");
    env::set_current_dir("../ucelofka-webapp/").expect("Failed to change directory.");
    call_clean();
    call_build();
    env::set_current_dir(current).expect("Failed to change directry.");
}
