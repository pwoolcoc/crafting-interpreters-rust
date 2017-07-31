use std::fs::File;
use std::io::{self, stdin, stdout, Read, Write, BufReader, BufRead};

use scanner::Scanner;
use token::Token;

mod scanner;
mod token;

pub fn run_file(filename: &str) {
    println!("running file {}", filename);
    let mut src = String::new();
    let mut f = File::open(filename).unwrap();
    f.read_to_string(&mut src);
    run(&src);
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

        run(&line);
    }
}

pub fn run(src: &str) {
    let scanner = Scanner::new(src);
    let tokens = scanner.scan();

    for token in tokens {
        println!("{:?}", token);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
