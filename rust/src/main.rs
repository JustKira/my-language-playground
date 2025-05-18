mod scanner;

use crate::scanner::*;

use std::{env, error::Error, fs, process::exit};

fn run(source: &str) {
    let scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    todo!()
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    run(&content);

    Ok(())
}

fn run_repl() -> Result<(), Box<dyn Error>> {
    use std::io::{self, Write};

    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        if io::stdin().read_line(&mut buffer)? == 0 {
            break;
        }

        if buffer.trim().is_empty() {
            continue;
        }

        run(&buffer);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl()?,
        2 => run_file(&args[1])?,
        _ => {
            eprintln!("Usage: jlox [script]");
            exit(64);
        }
    }

    Ok(())
}
