//! Tokenizer module for the LCI LOLCODE interpreter.
//!
//! The tokenizer converts lexemes into tokens with semantic meaning.

use crate::error::Result;
use crate::lexer::{Lexeme, LexemeList};

/// Token types in LOLCODE
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f32),
    String(String),
    Identifier(String),
    Boolean(bool),
    
    // Special keywords
    It,              // IT - implicit variable
    ItzLiekA,        // ITZ LIEK A - inherited object
    Noob,            // NOOB - nil
    Numbr,           // NUMBR - integer type
    Numbar,          // NUMBAR - float type
    Troof,           // TROOF - boolean type
    Yarn,            // YARN - string type
    Bukkit,          // BUKKIT - array type
    
    // Control
    Eof,
    Newline,
    
    // Program structure
    Hai,             // HAI - start of program
    KThxBye,         // KTHXBYE - end of program
    
    // Variables
    HasA,            // HAS A - variable declaration
    HasAn,           // HAS AN - variable declaration
    ItzA,            // ITZ A - type initialization
    Itz,             // ITZ - value initialization
    RNoob,           // R NOOB - deallocation
    R,               // R - assignment
    AnYr,            // AN YR - function argument separator
    An,              // AN - built-in function argument separator
    
    // Arithmetic operators
    SumOf,           // SUM OF - addition
    DiffOf,          // DIFF OF - subtraction
    ProduktOf,       // PRODUKT OF - multiplication
    QuoshuntOf,      // QUOSHUNT OF - division
    ModOf,           // MOD OF - modulo
    BiggrOf,         // BIGGR OF - max/greater
    SmallrOf,        // SMALLR OF - min/less
    
    // Boolean operators
    BothOf,          // BOTH OF - logical AND
    EitherOf,        // EITHER OF - logical OR
    WonOf,           // WON OF - logical XOR
    Not,             // NOT - logical NOT
    Mkay,            // MKAY - infinite arity delimiter
    AllOf,           // ALL OF - infinite arity AND
    AnyOf,           // ANY OF - infinite arity OR
    
    // Comparison operators
    BothSaem,        // BOTH SAEM - equality
    Diffrint,        // DIFFRINT - inequality
    
    // Casting
    Maek,            // MAEK - cast
    A,               // A - cast target separator
    IsNowA,          // IS NOW A - in-place cast
    
    // I/O
    Visible,         // VISIBLE - print to stdout
    Invisible,       // INVISIBLE - print to stderr
    Smoosh,          // SMOOSH - string concatenation
    Bang,            // ! - suppress newline in output
    Gimmeh,          // GIMMEH - input
    
    // Conditionals
    ORly,            // O RLY? - conditional
    YaRly,           // YA RLY - true branch
    Mebbe,           // MEBBE - else-if branch
    NoWai,           // NO WAI - false branch
    Oic,             // OIC - end conditional/switch
    
    // Switch
    Wtf,             // WTF? - switch
    Omg,             // OMG - case
    OmgWtf,          // OMGWTF - default case
    
    // Control flow
    Gtfo,            // GTFO - break/return
    
    // Loops
    ImInYr,          // IM IN YR - loop start
    Uppin,           // UPPIN - increment
    Nerfin,          // NERFIN - decrement
    Yr,              // YR - function/loop name delimiter
    Til,             // TIL - loop until condition
    Wile,            // WILE - loop while condition
    ImOuttaYr,       // IM OUTTA YR - loop end
    
    // Functions
    HowIz,           // HOW IZ - function definition start
    Iz,              // IZ - function scope delimiter
    IfUSaySo,        // IF U SAY SO - function definition end
    FoundYr,         // FOUND YR - return with value
    
    // Indirect access
    Srs,             // SRS - indirect variable access
    ApostropheZ,     // 'Z - array slot access
    
    // Alternate array syntax
    OHaiIm,          // O HAI IM - alternate array declaration
    ImLiek,          // IM LIEK - alternate inheritance
    Kthx,            // KTHX - end alternate array
}

/// A token with its type, image, and location information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub image: String,
    pub fname: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, image: String, fname: String, line: usize) -> Self {
        Self {
            token_type,
            image,
            fname,
            line,
        }
    }
}

/// Convert lexemes into tokens
pub fn tokenize_lexemes(lexemes: &LexemeList) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < lexemes.lexemes.len() {
        let lexeme = &lexemes.lexemes[i];
        let image = &lexeme.image;

        // Check for multi-lexeme keywords first
        if let Some((token_type, consumed)) = try_multi_lexeme_keyword(&lexemes.lexemes[i..]) {
            tokens.push(Token::new(
                token_type,
                image.clone(),
                lexeme.fname.clone(),
                lexeme.line,
            ));
            i += consumed;
            continue;
        }

        // Single lexeme tokens
        let token_type = match image.as_str() {
            "\n" => TokenType::Newline,
            "!" => TokenType::Bang,
            "'Z" => TokenType::ApostropheZ,
            
            "IT" => TokenType::It,
            "NOOB" => TokenType::Noob,
            "NUMBR" => TokenType::Numbr,
            "NUMBAR" => TokenType::Numbar,
            "TROOF" => TokenType::Troof,
            "YARN" => TokenType::Yarn,
            "BUKKIT" => TokenType::Bukkit,
            "HAI" => TokenType::Hai,
            "KTHXBYE" => TokenType::KThxBye,
            "R" => TokenType::R,
            "AN" => TokenType::An,
            "NOT" => TokenType::Not,
            "MKAY" => TokenType::Mkay,
            "MAEK" => TokenType::Maek,
            "A" => TokenType::A,
            "VISIBLE" => TokenType::Visible,
            "INVISIBLE" => TokenType::Invisible,
            "SMOOSH" => TokenType::Smoosh,
            "GIMMEH" => TokenType::Gimmeh,
            "MEBBE" => TokenType::Mebbe,
            "OIC" => TokenType::Oic,
            "OMG" => TokenType::Omg,
            "OMGWTF" => TokenType::OmgWtf,
            "GTFO" => TokenType::Gtfo,
            "UPPIN" => TokenType::Uppin,
            "NERFIN" => TokenType::Nerfin,
            "YR" => TokenType::Yr,
            "TIL" => TokenType::Til,
            "WILE" => TokenType::Wile,
            "IZ" => TokenType::Iz,
            "SRS" => TokenType::Srs,
            "KTHX" => TokenType::Kthx,
            
            // Check for boolean literals
            "WIN" => TokenType::Boolean(true),
            "FAIL" => TokenType::Boolean(false),
            
            _ => {
                // Check if it's a string literal
                if image.starts_with('"') {
                    let content = parse_string_literal(image)?;
                    TokenType::String(content)
                }
                // Check if it's an integer
                else if let Ok(val) = image.parse::<i64>() {
                    TokenType::Integer(val)
                }
                // Check if it's a float
                else if let Ok(val) = image.parse::<f32>() {
                    TokenType::Float(val)
                }
                // Otherwise it's an identifier
                else {
                    TokenType::Identifier(image.clone())
                }
            }
        };

        tokens.push(Token::new(
            token_type,
            image.clone(),
            lexeme.fname.clone(),
            lexeme.line,
        ));
        i += 1;
    }

    // Add EOF token
    let last_lexeme = lexemes.lexemes.last();
    if let Some(lex) = last_lexeme {
        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            lex.fname.clone(),
            lex.line,
        ));
    }

    Ok(tokens)
}

/// Try to match multi-lexeme keywords
/// Returns (TokenType, number of lexemes consumed) if matched
fn try_multi_lexeme_keyword(lexemes: &[Lexeme]) -> Option<(TokenType, usize)> {
    if lexemes.is_empty() {
        return None;
    }

    // Helper to join multiple lexeme images
    let join_images = |count: usize| -> String {
        lexemes[..count]
            .iter()
            .map(|l| l.image.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    };

    // Try 5-lexeme keywords
    if lexemes.len() >= 5 {
        match join_images(5).as_str() {
            "IM OUTTA YR" => return Some((TokenType::ImOuttaYr, 5)),
            _ => {}
        }
    }

    // Try 4-lexeme keywords
    if lexemes.len() >= 4 {
        match join_images(4).as_str() {
            "IF U SAY SO" => return Some((TokenType::IfUSaySo, 4)),
            "ITZ LIEK A" => return Some((TokenType::ItzLiekA, 4)),
            _ => {}
        }
    }

    // Try 3-lexeme keywords
    if lexemes.len() >= 3 {
        match join_images(3).as_str() {
            "O HAI IM" => return Some((TokenType::OHaiIm, 3)),
            "IS NOW A" => return Some((TokenType::IsNowA, 3)),
            "IM IN YR" => return Some((TokenType::ImInYr, 3)),
            "O RLY?" => return Some((TokenType::ORly, 3)),
            "YA RLY" => return Some((TokenType::YaRly, 3)),
            "NO WAI" => return Some((TokenType::NoWai, 3)),
            "IM LIEK" => return Some((TokenType::ImLiek, 3)),
            _ => {}
        }
    }

    // Try 2-lexeme keywords
    if lexemes.len() >= 2 {
        match join_images(2).as_str() {
            "HAS A" => return Some((TokenType::HasA, 2)),
            "HAS AN" => return Some((TokenType::HasAn, 2)),
            "ITZ A" => return Some((TokenType::ItzA, 2)),
            "R NOOB" => return Some((TokenType::RNoob, 2)),
            "AN YR" => return Some((TokenType::AnYr, 2)),
            "SUM OF" => return Some((TokenType::SumOf, 2)),
            "DIFF OF" => return Some((TokenType::DiffOf, 2)),
            "PRODUKT OF" => return Some((TokenType::ProduktOf, 2)),
            "QUOSHUNT OF" => return Some((TokenType::QuoshuntOf, 2)),
            "MOD OF" => return Some((TokenType::ModOf, 2)),
            "BIGGR OF" => return Some((TokenType::BiggrOf, 2)),
            "SMALLR OF" => return Some((TokenType::SmallrOf, 2)),
            "BOTH OF" => return Some((TokenType::BothOf, 2)),
            "EITHER OF" => return Some((TokenType::EitherOf, 2)),
            "WON OF" => return Some((TokenType::WonOf, 2)),
            "ALL OF" => return Some((TokenType::AllOf, 2)),
            "ANY OF" => return Some((TokenType::AnyOf, 2)),
            "BOTH SAEM" => return Some((TokenType::BothSaem, 2)),
            "HOW IZ" => return Some((TokenType::HowIz, 2)),
            "FOUND YR" => return Some((TokenType::FoundYr, 2)),
            "WTF?" => return Some((TokenType::Wtf, 2)),
            _ => {}
        }
    }

    None
}

/// Parse a string literal, handling escape sequences
fn parse_string_literal(s: &str) -> Result<String> {
    if !s.starts_with('"') {
        return Ok(s.to_string());
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 1; // Skip opening quote

    while i < chars.len() {
        if chars[i] == '"' {
            break;
        }

        if chars[i] == ':' && i + 1 < chars.len() {
            // Handle escape sequences
            match chars[i + 1] {
                ')' => result.push('\n'),
                '>' => result.push('\t'),
                'o' => result.push('\x07'), // Bell
                '"' => result.push('"'),
                ':' => result.push(':'),
                _ => {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                }
            }
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    Ok(result)
}
