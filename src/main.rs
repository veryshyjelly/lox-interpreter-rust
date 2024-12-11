use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use ast::Expression;
use evaluate::environment::Env;
use evaluate::Eval;
use evaluate::Object;
use evaluate::RuntimeError;
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
            evaluate(&scanner)?;
        }
        "run" => {
            let scanner = tokenize(file_contents, false)?;
            let parser = parse(&scanner, false)?;
            let mut env = vec![Env::default()];
            for d in &parser.program.unwrap().declarations {
                catch_err(d.evaluate(&mut env));
            }
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
        match Expression::parse(&scanner.tokens) {
            Ok((e, _)) => write!(io::stdout(), "{e}")?,
            Err(_) => {
                writeln!(
                    io::stderr(),
                    "[line {}] Error at '{}': {}",
                    err.tok.line,
                    err.tok.lexeme,
                    err.err
                )?;
                exit(65);
            }
        }
    } else if debug {
        if let Some(expr) = &parser.program {
            write!(io::stdout(), "{}", expr)?;
        }
    }
    Ok(parser)
}

fn evaluate<'a>(scanner: &'a Scanner) -> std::io::Result<()> {
    let expr = Expression::parse(&scanner.tokens);
    match expr {
        Ok((ex, _)) => {
            let val = catch_err(ex.evaluate(&mut vec![Env::default()]));
            write!(io::stdout(), "{val}")
        }
        Err(err) => writeln!(
            io::stderr(),
            "[line {}] Error at '{}': {}",
            err.tok.line,
            err.tok.lexeme,
            err.err
        ),
    }
}

fn catch_err(val: Result<Object, RuntimeError>) -> Object {
    match val {
        Ok(v) => v,
        Err(e) => {
            write!(io::stderr(), "{}", e.err);
            exit(70)
        }
    }
}
