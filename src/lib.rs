//! LCI - A LOLCODE interpreter library
//!
//! This library provides all the components needed to lex, tokenize, parse,
//! and interpret LOLCODE programs.

pub mod error;
pub mod lexer;
pub mod tokenizer;
pub mod parser;
pub mod interpreter;

// Re-export main types
pub use error::{LciError, Result};
pub use lexer::{Lexeme, LexemeList, scan_buffer};
pub use tokenizer::{Token, TokenType, tokenize_lexemes};
pub use parser::{MainNode, Statement, Expression, parse_main_node};
pub use interpreter::{Interpreter, Value};
