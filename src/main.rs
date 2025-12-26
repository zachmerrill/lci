//! LCI - A LOLCODE interpreter written in Rust
//!
//! This is a modern Rust implementation of the LCI LOLCODE interpreter,
//! designed to be correct, portable, fast, and well-documented.

use clap::Parser as ClapParser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

mod error;
mod lexer;
mod tokenizer;
mod parser;
mod interpreter;

use error::{LciError, Result};

/// LCI - A LOLCODE interpreter
#[derive(ClapParser, Debug)]
#[command(name = "lci")]
#[command(version = "0.10.5")]
#[command(about = "Interpret LOLCODE files", long_about = None)]
struct Args {
    /// LOLCODE file(s) to interpret. Use '-' for stdin
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Display version information
    #[arg(short = 'v', long = "version")]
    version: bool,
}

fn main() {
    let args = Args::parse();

    // Handle version flag
    if args.version {
        println!("lci {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    // If no files specified, show help
    if args.files.is_empty() {
        eprintln!("Usage: lci [FILE]...");
        eprintln!("Interpret FILE(s) as LOLCODE. Let FILE be '-' for stdin.");
        eprintln!("  -h, --help       Display this help");
        eprintln!("  -v, --version    Display version information");
        process::exit(1);
    }

    // Process each file
    for file_path in &args.files {
        if let Err(e) = process_file(file_path) {
            eprintln!("{}", e);
            process::exit(e.exit_code());
        }
    }
}

/// Process a single LOLCODE file
fn process_file(path: &PathBuf) -> Result<()> {
    // Read file content
    let (content, fname) = if path.to_str() == Some("-") {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| LciError::Io(e))?;
        (buffer, "stdin".to_string())
    } else {
        // Read from file
        let content = fs::read_to_string(path).map_err(|_| {
            LciError::ErrorOpeningFile(path.to_string_lossy().to_string())
        })?;
        (content, path.to_string_lossy().to_string())
    };

    // Process shebang if present
    let content = remove_shebang(&content);

    // Remove UTF-8 BOM if present and add it to output
    let content = if content.starts_with('\u{FEFF}') {
        print!("\u{FEFF}");
        content.trim_start_matches('\u{FEFF}')
    } else {
        &content
    };

    // Lexing
    let lexemes = lexer::scan_buffer(content, &fname)?;

    // Tokenizing
    let tokens = tokenizer::tokenize_lexemes(&lexemes)?;

    // Parsing
    let ast = parser::parse_main_node(&tokens)?;

    // Interpreting
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&ast)?;

    Ok(())
}

/// Remove shebang line if present
fn remove_shebang(content: &str) -> String {
    if content.starts_with("#!") {
        // Find the end of the first line
        if let Some(pos) = content.find('\n') {
            // Replace shebang line with spaces to preserve line numbers
            let spaces = " ".repeat(pos);
            format!("{}{}", spaces, &content[pos..])
        } else {
            content.to_string()
        }
    } else {
        content.to_string()
    }
}
