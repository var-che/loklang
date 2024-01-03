mod expr;
mod scanner;
use crate::scanner::*;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(content) => return run(&content),
        Err(err) => return Err(err.to_string()),
    }
}

fn run(content: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(content);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    return Ok(());
}
fn run_promt() -> Result<(), String> {
    loop {
        print!("> ");
        let mut buffer = String::new();
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush stdout".to_string()),
        }
        let stdin = io::stdin();
        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    break;
                }
            }
            Err(_) => return Err("Could not read the line".to_string()),
        }
        println!("ECHO: {}", buffer);
        match run(&buffer) {
            Ok(_) => (),
            Err(_msg) => return Err("Problem with the buffer".to_string()),
        }
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_promt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR\n{}", msg);
                exit(1);
            }
        }
    }
}
