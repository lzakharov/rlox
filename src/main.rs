use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod error;
mod object;
mod scanner;
mod token;

use error::Error;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    }
    if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt().expect("Could not run prompt");
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    match run(&source) {
        Ok(_) => {}
        Err(e) => {
            e.report("");
            std::process::exit(65);
        }
    };
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    print_single_line("> ");

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            match run(&line) {
                Ok(_) => {}
                Err(e) => {
                    e.report("");
                }
            };
        } else {
            break;
        }

        print_single_line("> ");
    }

    Ok(())
}

fn print_single_line(line: &str) {
    print!("{}", line);
    io::stdout().flush().expect("Could not flush stdout");
}

fn run(source: &str) -> Result<(), Error> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
