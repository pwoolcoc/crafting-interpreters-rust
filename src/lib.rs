#[macro_use] extern crate error_chain;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write, BufReader, BufRead};
use std::process;

pub use errors::*;
use scanner::Scanner;

mod errors;
mod scanner;
mod token;
mod token_type;
mod literal;

pub fn run_file(filename: &str) {
    let mut src = String::new();
    let mut f = File::open(filename).unwrap();
    if let Err(e) = f.read_to_string(&mut src) {
        eprintln!("{}: Error reading from file {}", e, &filename);
        process::exit(65);
    }
    if let Err(e) = run(&src) {
        eprintln!("{}", e.description());
        process::exit(65);
    }
}

pub fn run_prompt() {
    let handle = stdin();
    let handle = handle.lock();
    let buf_handle = BufReader::new(handle);
    let mut lines = buf_handle.lines();

    loop {
        print!("> ");
        let _ = stdout().flush().unwrap();

        let line = if let Some(line) = lines.next() {
            line.unwrap()
        } else {
            break;
        };

        if let Err(e) = run(&line) {
            eprintln!("{}", e.description());
        }
    }
}

pub fn run(src: &str) -> Result<()> {
    let scanner = Scanner::new(src);
    let tokens = scanner.scan();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
