use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use parse::Parser;
use scan::Scanner;

mod ast;
mod parse;
mod scan;
mod token;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0])?;
        return Ok(());
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let mut scanner = Scanner::new(file_contents);
    scanner.scan();

    match command.as_str() {
        "tokenize" => {
            for ele in scanner.errors.iter() {
                writeln!(io::stderr(), "[line {}] Error: {}", ele.line, ele.tok)?;
            }
            for ele in scanner.tokens.iter() {
                writeln!(io::stdout(), "{ele}")?;
            }
            if !scanner.errors.is_empty() {
                exit(65);
            }
            Ok(())
        }
        "parse" => {
            let mut parser = Parser::new(&scanner.tokens);
            parser.parse();
            Ok(())
        }
        _ => writeln!(io::stderr(), "Unknown command: {}", command),
    }
}
