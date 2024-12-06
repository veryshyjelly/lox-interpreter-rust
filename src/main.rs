use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use scan::Scanner;

mod scan;
mod token;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return Ok(());
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                scanner.scan();
                for ele in scanner.errors.iter() {
                    writeln!(
                        io::stderr(),
                        "[line {}] Error: Unexpected character: {}",
                        ele.line,
                        ele.tok
                    )?;
                }
                for ele in scanner.tokens.iter() {
                    writeln!(io::stdout(), "{ele}")?;
                }
                if !scanner.errors.is_empty() {
                    exit(65);
                }
                Ok(())
            } else {
                writeln!(io::stdout(), "EOF  null")
            }
        }
        _ => writeln!(io::stderr(), "Unknown command: {}", command),
    }
}
