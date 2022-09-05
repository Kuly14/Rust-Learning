use std::process::Command;
use std::io;

fn main() {
       let result = Command::new("git")
        .args(["add", "."]).output().expect("failed");

    match result.status.success() {
        true => {
            ()
        }
        false => {
            println!("Failed to add new changes");
            std::process::exit(1);
        }
    }

    let mut name = String::new();

    println!("What do you want to name your commit: ");
    io::stdin().read_line(&mut name)
        .expect("Failed to read string");

    let result = Command::new("git")
        .args(["commit", "-m", &name]).output().expect("failed");

    match result.status.success() {
        true => {
            ()
        }
        false => {
            println!("Failed to commit");
            std::process::exit(1);
        }
    }

    println!("To what branch do you want to push?");
    let mut branch = String::new();

    io::stdin().read_line(&mut branch)
        .expect("failed to read string");

    let result = Command::new("git")
        .args(["push", "origin", &branch]).output().unwrap();

    match result.status.success() {
        true => {
            ()
        }
        false => {
            println!("Failed to push to main");
            std::process::exit(1);
        }
    }
}

