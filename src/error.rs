//! Error types for the LCI LOLCODE interpreter.
//!
//! This module defines all error types that can occur during lexing, tokenizing,
//! parsing, and interpreting LOLCODE programs.

use thiserror::Error;

/// Main error type for the LCI interpreter
#[derive(Error, Debug)]
pub enum LciError {
    // Main body errors (100 block)
    #[error("Error opening file '{0}'")]
    ErrorOpeningFile(String),
    
    #[error("Error closing file '{0}'")]
    ErrorClosingFile(String),

    // Lexer errors (200 block)
    #[error("{0}:{1}: a line with continuation may not be followed by an empty line")]
    LineContinuation(String, usize),
    
    #[error("{0}:{1}: multiple line comment may not appear on the same line as code")]
    MultipleLineComment(String, usize),
    
    #[error("{0}:{1}: expected token delimiter after string literal")]
    ExpectedTokenDelimiter(String, usize),

    // Tokenizer errors (300 block)
    #[error("{0}:{1}: expected floating point decimal value")]
    ExpectedFloatingPoint(String, usize),
    
    #[error("{0}:{1}: expected integer value")]
    ExpectedInteger(String, usize),
    
    #[error("{0}:{1}: unknown token at: {2}")]
    UnknownToken(String, usize, String),

    // Parser errors (400 block)
    #[error("{0}:{1}: unable to delete unknown identifier type")]
    UnknownIdentifierType(String, usize),
    
    #[error("unable to delete unknown statement type")]
    UnknownStatementType,
    
    #[error("unable to delete unknown expression type")]
    UnknownExpressionType,
    
    #[error("{0}:{1}: expected boolean at: {2}")]
    ExpectedBoolean(String, usize, String),
    
    #[error("{0}:{1}: expected integer at: {2}")]
    ExpectedIntegerToken(String, usize, String),
    
    #[error("{0}:{1}: expected float at: {2}")]
    ExpectedFloat(String, usize, String),
    
    #[error("{0}:{1}: expected string at: {2}")]
    ExpectedString(String, usize, String),
    
    #[error("{0}:{1}: expected constant value at: {2}")]
    ExpectedConstant(String, usize, String),
    
    #[error("{0}:{1}: expected type at: {2}")]
    ExpectedType(String, usize, String),
    
    #[error("{0}:{1}: expected identifier at: {2}")]
    ExpectedIdentifier(String, usize, String),
    
    #[error("{0}:{1}: expected {2} at: {3}")]
    ExpectedToken(String, usize, String, String),
    
    #[error("{0}:{1}: invalid operator at: {2}")]
    InvalidOperator(String, usize, String),
    
    #[error("{0}:{1}: expected expression at: {2}")]
    ExpectedExpression(String, usize, String),
    
    #[error("{0}:{1}: expected end of expression at: {2}")]
    ExpectedEndOfExpression(String, usize, String),
    
    #[error("{0}:{1}: expected end of statement at: {2}")]
    ExpectedEndOfStatement(String, usize, String),
    
    #[error("{0}:{1}: cannot use an interpolated string as an OMG literal at: {2}")]
    CannotUseStrAsLiteral(String, usize, String),
    
    #[error("{0}:{1}: OMG literal must be unique at: {2}")]
    LiteralMustBeUnique(String, usize, String),
    
    #[error("{0}:{1}: expected loop name at: {2}")]
    ExpectedLoopName(String, usize, String),
    
    #[error("{0}:{1}: expected {2} or {3} at: {4}")]
    ExpectedEitherToken(String, usize, String, String, String),
    
    #[error("{0}:{1}: expected unary function at: {2}")]
    ExpectedUnaryFunction(String, usize, String),
    
    #[error("{0}:{1}: expected matching loop name at: {2}")]
    ExpectedMatchingLoopName(String, usize, String),
    
    #[error("{0}:{1}: expected statement at: {2}")]
    ExpectedStatement(String, usize, String),
    
    #[error("unhandled string detected")]
    UnhandledString,

    // Interpreter errors (500 block)
    #[error("{0}:{1}: invalid identifier type at: {2}")]
    InvalidIdentifierType(String, usize, String),
    
    #[error("{0}:{1}: unable to store variable: {2}")]
    UnableToStoreVariable(String, usize, String),
    
    #[error("{0}:{1}: variable does not exist: {2}")]
    VariableDoesNotExist(String, usize, String),
    
    #[error("Cannot implicitly cast nil")]
    CannotImplicitlyCastNil,
    
    #[error("Cannot cast function to boolean value")]
    CannotCastFunctionToBoolean,
    
    #[error("Cannot cast array to boolean value")]
    CannotCastArrayToBoolean,
    
    #[error("Unknown value type encountered during boolean cast")]
    UnknownValueDuringBooleanCast,
    
    #[error("Unable to cast value")]
    UnableToCastValue,
    
    #[error("Expected integer value")]
    ExpectedIntegerValue,
    
    #[error("Cannot cast function to integer value")]
    CannotCastFunctionToInteger,
    
    #[error("Cannot cast array to integer value")]
    CannotCastArrayToInteger,
    
    #[error("Unknown value type encountered during integer cast")]
    UnknownValueDuringIntegerCast,
    
    #[error("Expected floating point decimal value")]
    ExpectedDecimal,
    
    #[error("Cannot cast function to floating point decimal value")]
    CannotCastFunctionToDecimal,
    
    #[error("Cannot cast array to floating point decimal value")]
    CannotCastArrayToDecimal,
    
    #[error("Unknown value type encountered during floating point decimal cast")]
    UnknownValueDuringDecimalCast,
    
    #[error("Cannot cast boolean to string value")]
    CannotCastBooleanToString,
    
    #[error("Expected closing parenthesis after :(")]
    ExpectedClosingParen,
    
    #[error("Please supply a valid hexadecimal number")]
    InvalidHexNumber,
    
    #[error("Code point is supposed to be positive")]
    CodePointMustBePositive,
    
    #[error("Expected closing square bracket after :[")]
    ExpectedClosingSquareBracket,
    
    #[error("Expected closing curly brace after :{{")]
    ExpectedClosingCurlyBrace,
    
    #[error("{0}:{1}: variable is not an array: {2}")]
    VariableNotAnArray(String, usize, String),
    
    #[error("Cannot cast function to string value")]
    CannotCastFunctionToString,
    
    #[error("Cannot cast array to string value")]
    CannotCastArrayToString,
    
    #[error("Unknown value type encountered during string cast")]
    UnknownValueDuringStringCast,
    
    #[error("Unknown cast type")]
    UnknownCastType,
    
    #[error("{0}:{1}: undefined function at: {2}")]
    UndefinedFunction(String, usize, String),
    
    #[error("{0}:{1}: incorrect number of arguments supplied to: {2}")]
    IncorrectNumberOfArguments(String, usize, String),
    
    #[error("Invalid return type")]
    InvalidReturnType,
    
    #[error("Unknown constant type")]
    UnknownConstantType,
    
    #[error("Division by zero undefined")]
    DivisionByZero,
    
    #[error("Invalid operand type")]
    InvalidOperandType,
    
    #[error("Invalid boolean operation type")]
    InvalidBooleanOperationType,
    
    #[error("Invalid equality operation type")]
    InvalidEqualityOperationType,
    
    #[error("{0}:{1}: redefinition of existing variable at: {2}")]
    RedefinitionOfVariable(String, usize, String),
    
    #[error("Unknown declaration type")]
    InvalidDeclarationType,
    
    #[error("Invalid type")]
    InvalidType,
    
    #[error("{0}:{1}: function name already used by existing variable at: {2}")]
    FunctionNameUsedByVariable(String, usize, String),
    
    #[error("{0}:{1}: cannot cast value to array at: {2}")]
    CannotCastValueToArray(String, usize, String),

    // I/O errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for LCI operations
pub type Result<T> = std::result::Result<T, LciError>;

impl LciError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            // Main body errors (100 block)
            Self::ErrorOpeningFile(_) => 100,
            Self::ErrorClosingFile(_) => 101,
            
            // Lexer errors (200 block)
            Self::LineContinuation(_, _) => 200,
            Self::MultipleLineComment(_, _) => 201,
            Self::ExpectedTokenDelimiter(_, _) => 202,
            
            // Tokenizer errors (300 block)
            Self::ExpectedFloatingPoint(_, _) => 300,
            Self::ExpectedInteger(_, _) => 301,
            Self::UnknownToken(_, _, _) => 302,
            
            // Parser errors (400 block)
            Self::UnknownIdentifierType(_, _) => 400,
            Self::UnknownStatementType => 401,
            Self::UnknownExpressionType => 402,
            Self::ExpectedBoolean(_, _, _) => 403,
            Self::ExpectedIntegerToken(_, _, _) => 404,
            Self::ExpectedFloat(_, _, _) => 405,
            Self::ExpectedString(_, _, _) => 406,
            Self::ExpectedConstant(_, _, _) => 407,
            Self::ExpectedType(_, _, _) => 408,
            Self::ExpectedIdentifier(_, _, _) => 409,
            Self::ExpectedToken(_, _, _, _) => 410,
            Self::InvalidOperator(_, _, _) => 411,
            Self::ExpectedExpression(_, _, _) => 412,
            Self::ExpectedEndOfExpression(_, _, _) => 413,
            Self::ExpectedEndOfStatement(_, _, _) => 414,
            Self::CannotUseStrAsLiteral(_, _, _) => 415,
            Self::LiteralMustBeUnique(_, _, _) => 416,
            Self::ExpectedLoopName(_, _, _) => 417,
            Self::ExpectedEitherToken(_, _, _, _, _) => 418,
            Self::ExpectedUnaryFunction(_, _, _) => 419,
            Self::ExpectedMatchingLoopName(_, _, _) => 420,
            Self::ExpectedStatement(_, _, _) => 421,
            Self::UnhandledString => 422,
            
            // Interpreter errors (500 block)
            Self::InvalidIdentifierType(_, _, _) => 500,
            Self::UnableToStoreVariable(_, _, _) => 501,
            Self::VariableDoesNotExist(_, _, _) => 502,
            Self::CannotImplicitlyCastNil => 503,
            Self::CannotCastFunctionToBoolean => 504,
            Self::CannotCastArrayToBoolean => 505,
            Self::UnknownValueDuringBooleanCast => 506,
            Self::UnableToCastValue => 507,
            Self::ExpectedIntegerValue => 508,
            Self::CannotCastFunctionToInteger => 509,
            Self::CannotCastArrayToInteger => 510,
            Self::UnknownValueDuringIntegerCast => 511,
            Self::ExpectedDecimal => 512,
            Self::CannotCastFunctionToDecimal => 513,
            Self::CannotCastArrayToDecimal => 514,
            Self::UnknownValueDuringDecimalCast => 515,
            Self::CannotCastBooleanToString => 516,
            Self::ExpectedClosingParen => 517,
            Self::InvalidHexNumber => 518,
            Self::CodePointMustBePositive => 519,
            Self::ExpectedClosingSquareBracket => 520,
            Self::ExpectedClosingCurlyBrace => 521,
            Self::VariableNotAnArray(_, _, _) => 522,
            Self::CannotCastFunctionToString => 523,
            Self::CannotCastArrayToString => 524,
            Self::UnknownValueDuringStringCast => 525,
            Self::UnknownCastType => 526,
            Self::UndefinedFunction(_, _, _) => 527,
            Self::IncorrectNumberOfArguments(_, _, _) => 528,
            Self::InvalidReturnType => 529,
            Self::UnknownConstantType => 530,
            Self::DivisionByZero => 531,
            Self::InvalidOperandType => 532,
            Self::InvalidBooleanOperationType => 533,
            Self::InvalidEqualityOperationType => 534,
            Self::RedefinitionOfVariable(_, _, _) => 535,
            Self::InvalidDeclarationType => 536,
            Self::InvalidType => 537,
            Self::FunctionNameUsedByVariable(_, _, _) => 538,
            Self::CannotCastValueToArray(_, _, _) => 539,
            
            // I/O errors
            Self::Io(_) => 1,
        }
    }
}
