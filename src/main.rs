use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use parse::Parser;
use scan::Scanner;

mod ast;
mod display;
mod evaluate;
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

    match command.as_str() {
        "tokenize" => {
            tokenize(file_contents, true)?;
        }
        "parse" => {
            let scanner = tokenize(file_contents, false)?;
            parse(&scanner, true)?;
        }
        "evaluate" => {
            let scanner = tokenize(file_contents, false)?;
            let parser = parse(&scanner, false)?;
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command)?;
        }
    }
    Ok(())
}

fn tokenize(file_contents: String, debug: bool) -> std::io::Result<Scanner> {
    let mut scanner = Scanner::new(file_contents);
    scanner.scan();
    for ele in scanner.errors.iter() {
        writeln!(io::stderr(), "[line {}] Error: {}", ele.line, ele.tok)?;
    }
    if debug {
        for ele in scanner.tokens.iter() {
            writeln!(io::stdout(), "{ele}")?;
        }
    }
    if !scanner.errors.is_empty() {
        exit(65);
    }
    Ok(scanner)
}

fn parse<'a>(scanner: &'a Scanner, debug: bool) -> std::io::Result<Parser> {
    let mut parser = Parser::new(&scanner.tokens);
    if let Err(err) = parser.parse() {
        writeln!(
            io::stderr(),
            "[line {}] Error at '{}': {}",
            err.tok.line,
            err.tok.lexeme,
            err.err
        )?;
        exit(65);
    } else if debug {
        if let Some(expr) = &parser.program {
            println!("{}", expr);
        }
    }
    Ok(parser)
}
