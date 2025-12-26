//! Lexer module for the LCI LOLCODE interpreter.
//!
//! The lexer takes a character buffer and splits it into lexemes - groups of
//! contiguous characters stripped of surrounding whitespace.

use crate::error::Result;

/// A lexeme representing a group of characters from the source
#[derive(Debug, Clone)]
pub struct Lexeme {
    /// The string content of the lexeme
    pub image: String,
    /// The file name where this lexeme originated
    pub fname: String,
    /// The line number where this lexeme occurred
    pub line: usize,
}

impl Lexeme {
    /// Create a new lexeme
    pub fn new(image: String, fname: String, line: usize) -> Self {
        Self { image, fname, line }
    }
}

/// A list of lexemes
#[derive(Debug)]
pub struct LexemeList {
    /// The lexemes in this list
    pub lexemes: Vec<Lexeme>,
}

impl LexemeList {
    /// Create a new empty lexeme list
    pub fn new() -> Self {
        Self {
            lexemes: Vec::new(),
        }
    }

    /// Add a lexeme to the list
    pub fn add(&mut self, lexeme: Lexeme) {
        self.lexemes.push(lexeme);
    }
}

impl Default for LexemeList {
    fn default() -> Self {
        Self::new()
    }
}

/// Scan a buffer and convert it into a list of lexemes
///
/// This function processes the input buffer character by character, handling:
/// - Whitespace and newlines
/// - Comments (BTW, OBTW/TLDR)
/// - String literals with escape sequences
/// - Special tokens like commas (soft newlines) and bangs
pub fn scan_buffer(buffer: &str, fname: &str) -> Result<LexemeList> {
    let mut list = LexemeList::new();
    let mut line = 1;
    let chars: Vec<char> = buffer.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        // Skip whitespace (except newlines)
        if ch == ' ' || ch == '\t' {
            i += 1;
            continue;
        }

        // Handle newlines
        if ch == '\n' {
            list.add(Lexeme::new("\n".to_string(), fname.to_string(), line));
            line += 1;
            i += 1;
            continue;
        }

        // Handle carriage returns
        if ch == '\r' {
            // Check for \r\n
            if i + 1 < chars.len() && chars[i + 1] == '\n' {
                list.add(Lexeme::new("\n".to_string(), fname.to_string(), line));
                line += 1;
                i += 2;
            } else {
                list.add(Lexeme::new("\n".to_string(), fname.to_string(), line));
                line += 1;
                i += 1;
            }
            continue;
        }

        // Comma (,) is a soft newline
        if ch == ',' {
            list.add(Lexeme::new("\n".to_string(), fname.to_string(), line));
            i += 1;
            continue;
        }

        // Bang (!) is its own lexeme
        if ch == '!' {
            list.add(Lexeme::new("!".to_string(), fname.to_string(), line));
            i += 1;
            continue;
        }

        // Apostrophe Z ('Z) is its own lexeme for array access
        if i + 1 < chars.len() && ch == '\'' && chars[i + 1] == 'Z' {
            list.add(Lexeme::new("'Z".to_string(), fname.to_string(), line));
            i += 2;
            continue;
        }

        // Handle string literals
        if ch == '"' {
            let mut string = String::from("\"");
            i += 1;
            
            while i < chars.len() {
                let c = chars[i];
                
                // Check for escaped characters with colon
                if c == ':' && i + 1 < chars.len() {
                    string.push(c);
                    i += 1;
                    string.push(chars[i]);
                    i += 1;
                    continue;
                }
                
                // End of string
                if c == '"' {
                    string.push(c);
                    i += 1;
                    break;
                }
                
                // Newline in string is an error, but we'll handle it gracefully
                if c == '\n' || c == '\r' {
                    break;
                }
                
                string.push(c);
                i += 1;
            }
            
            list.add(Lexeme::new(string, fname.to_string(), line));
            continue;
        }

        // Handle single-line comments (BTW)
        if i + 3 <= chars.len() {
            let word: String = chars[i..std::cmp::min(i + 3, chars.len())].iter().collect();
            if word == "BTW" {
                // Skip to end of line
                while i < chars.len() && chars[i] != '\n' && chars[i] != '\r' {
                    i += 1;
                }
                continue;
            }
        }

        // Handle multi-line comments (OBTW...TLDR)
        if i + 4 <= chars.len() {
            let word: String = chars[i..std::cmp::min(i + 4, chars.len())].iter().collect();
            if word == "OBTW" {
                // Find TLDR
                i += 4;
                
                while i < chars.len() {
                    if chars[i] == '\n' {
                        line += 1;
                    }
                    
                    // Check for TLDR
                    if i + 4 <= chars.len() {
                        let end_word: String = chars[i..std::cmp::min(i + 4, chars.len())].iter().collect();
                        if end_word == "TLDR" {
                            i += 4;
                            // Skip to end of line
                            while i < chars.len() && chars[i] != '\n' && chars[i] != '\r' {
                                i += 1;
                            }
                            break;
                        }
                    }
                    i += 1;
                }
                continue;
            }
        }

        // Handle line continuation (...)
        if i + 3 <= chars.len() {
            let word: String = chars[i..std::cmp::min(i + 3, chars.len())].iter().collect();
            if word == "..." {
                i += 3;
                // Skip following whitespace and newline
                while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
                    i += 1;
                }
                if i < chars.len() && (chars[i] == '\n' || chars[i] == '\r') {
                    if chars[i] == '\r' && i + 1 < chars.len() && chars[i + 1] == '\n' {
                        i += 2;
                    } else {
                        i += 1;
                    }
                    line += 1;
                }
                continue;
            }
        }

        // Collect regular lexeme (word)
        let mut word = String::new();
        while i < chars.len() {
            let c = chars[i];
            
            // Stop at whitespace, special characters, or string delimiters
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' || 
               c == ',' || c == '!' || c == '"' || c == '\'' {
                break;
            }
            
            word.push(c);
            i += 1;
        }

        if !word.is_empty() {
            list.add(Lexeme::new(word, fname.to_string(), line));
        }
    }

    Ok(list)
}
