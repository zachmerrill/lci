//! Parser module for the LCI LOLCODE interpreter.
//!
//! The parser converts tokens into an Abstract Syntax Tree (AST) that can be
//! interpreted.

use crate::error::{LciError, Result};
use crate::tokenizer::{Token, TokenType};
use std::collections::HashMap;

/// Main program node - root of the AST
#[derive(Debug, Clone)]
pub struct MainNode {
    pub version: String,
    pub statements: Vec<Statement>,
}

/// Statement types in LOLCODE
#[derive(Debug, Clone)]
pub enum Statement {
    /// Variable declaration (I HAS A var)
    Declaration {
        name: String,
        value: Option<Expression>,
    },
    /// Assignment (var R value)
    Assignment {
        target: String,
        value: Expression,
    },
    /// Print statement (VISIBLE expr)
    Print {
        expressions: Vec<Expression>,
        newline: bool,
        to_stderr: bool,
    },
    /// Input statement (GIMMEH var)
    Input {
        target: String,
    },
    /// If-then-else (O RLY?)
    IfThenElse {
        condition: Option<Expression>,
        then_block: Vec<Statement>,
        else_ifs: Vec<(Expression, Vec<Statement>)>,
        else_block: Option<Vec<Statement>>,
    },
    /// Loop (IM IN YR loop)
    Loop {
        label: String,
        operation: Option<String>,
        variable: Option<String>,
        condition: Option<(String, Expression)>,
        body: Vec<Statement>,
    },
    /// Function definition (HOW IZ I func)
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    /// Function call as statement
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    /// Return statement (FOUND YR expr)
    Return {
        value: Option<Expression>,
    },
    /// Break statement (GTFO)
    Break,
    /// Expression statement (evaluates expression and stores in IT)
    Expression(Expression),
}

/// Expression types in LOLCODE
#[derive(Debug, Clone)]
pub enum Expression {
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f32),
    /// String literal
    String(String),
    /// Boolean literal
    Boolean(bool),
    /// Nil value (NOOB)
    Nil,
    /// Variable reference
    Variable(String),
    /// Implicit variable (IT)
    ImplicitVariable,
    /// Binary operation
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Unary operation
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    /// Function call
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    /// Type cast (MAEK expr A type)
    Cast {
        expr: Box<Expression>,
        target_type: ValueType,
    },
}

/// Binary operators in LOLCODE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,      // SUM OF
    Sub,      // DIFF OF
    Mul,      // PRODUKT OF
    Div,      // QUOSHUNT OF
    Mod,      // MOD OF
    Max,      // BIGGR OF
    Min,      // SMALLR OF
    And,      // BOTH OF
    Or,       // EITHER OF
    Xor,      // WON OF
    Equal,    // BOTH SAEM
    NotEqual, // DIFFRINT
}

/// Unary operators in LOLCODE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not, // NOT
}

/// Value types in LOLCODE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Noob,   // NOOB - nil
    Troof,  // TROOF - boolean
    Numbr,  // NUMBR - integer
    Numbar, // NUMBAR - float
    Yarn,   // YARN - string
    Bukkit, // BUKKIT - array
}

/// Parse tokens into an AST
pub fn parse_main_node(tokens: &[Token]) -> Result<MainNode> {
    let mut parser = Parser::new(tokens);
    parser.parse_main()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    fn advance(&mut self) -> Option<&Token> {
        let pos = self.pos;
        self.pos += 1;
        self.tokens.get(pos)
    }

    fn expect(&mut self, expected: &TokenType) -> Result<&Token> {
        if let Some(token) = self.current() {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(expected) {
                return Ok(self.advance().unwrap());
            }
        }
        Err(LciError::ExpectedToken(
            String::new(),
            0,
            format!("{:?}", expected),
            String::new(),
        ))
    }

    fn skip_newlines(&mut self) {
        while let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Newline) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_main(&mut self) -> Result<MainNode> {
        // Expect HAI
        self.expect(&TokenType::Hai)?;
        self.skip_newlines();

        // Parse version (optional)
        let version = if let Some(token) = self.current() {
            if let TokenType::Float(v) = token.token_type {
                self.advance();
                v.to_string()
            } else if let TokenType::Integer(v) = token.token_type {
                self.advance();
                v.to_string()
            } else {
                "1.3".to_string()
            }
        } else {
            "1.3".to_string()
        };

        self.skip_newlines();

        // Parse statements until KTHXBYE
        let mut statements = Vec::new();
        while let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::KThxBye) {
                break;
            }
            if matches!(token.token_type, TokenType::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement()?);
        }

        self.expect(&TokenType::KThxBye)?;

        Ok(MainNode {
            version,
            statements,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        self.skip_newlines();

        if let Some(token) = self.current() {
            match &token.token_type {
                TokenType::HasA | TokenType::HasAn => {
                    return self.parse_declaration();
                }
                TokenType::Visible => {
                    return self.parse_print(false);
                }
                TokenType::Invisible => {
                    return self.parse_print(true);
                }
                TokenType::Gimmeh => {
                    return self.parse_input();
                }
                TokenType::Identifier(_) => {
                    // Could be assignment or expression statement
                    if let Some(next) = self.peek(1) {
                        if matches!(next.token_type, TokenType::R) {
                            return self.parse_assignment();
                        }
                    }
                    // If not assignment, treat as expression statement
                    // (could be function call or variable reference)
                }
                TokenType::ORly => {
                    return self.parse_if_then_else();
                }
                TokenType::ImInYr => {
                    return self.parse_loop();
                }
                TokenType::HowIz => {
                    return self.parse_function_def();
                }
                TokenType::FoundYr => {
                    return self.parse_return();
                }
                TokenType::Gtfo => {
                    self.advance();
                    self.skip_newlines();
                    return Ok(Statement::Break);
                }
                _ => {}
            }

            // Try to parse as expression statement
            let expr = self.parse_expression()?;
            self.skip_newlines();
            Ok(Statement::Expression(expr))
        } else {
            Err(LciError::ExpectedStatement(String::new(), 0, String::new()))
        }
    }

    fn parse_declaration(&mut self) -> Result<Statement> {
        self.advance(); // Skip HAS A/HAS AN
        
        let name = if let Some(token) = self.current() {
            if let TokenType::Identifier(name) = &token.token_type {
                let n = name.clone();
                self.advance();
                n
            } else {
                return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
            }
        } else {
            return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
        };

        let value = if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Itz) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            }
        } else {
            None
        };

        self.skip_newlines();
        Ok(Statement::Declaration { name, value })
    }

    fn parse_assignment(&mut self) -> Result<Statement> {
        let target = if let Some(token) = self.current() {
            if let TokenType::Identifier(name) = &token.token_type {
                let n = name.clone();
                self.advance();
                n
            } else {
                return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
            }
        } else {
            return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
        };

        self.expect(&TokenType::R)?;
        let value = self.parse_expression()?;
        self.skip_newlines();

        Ok(Statement::Assignment { target, value })
    }

    fn parse_print(&mut self, to_stderr: bool) -> Result<Statement> {
        self.advance(); // Skip VISIBLE/INVISIBLE

        let mut expressions = Vec::new();
        let mut newline = true;

        loop {
            if let Some(token) = self.current() {
                match &token.token_type {
                    TokenType::Newline | TokenType::Eof => break,
                    TokenType::Bang => {
                        newline = false;
                        self.advance();
                        break;
                    }
                    _ => {
                        expressions.push(self.parse_expression()?);
                    }
                }
            } else {
                break;
            }
        }

        self.skip_newlines();
        Ok(Statement::Print {
            expressions,
            newline,
            to_stderr,
        })
    }

    fn parse_input(&mut self) -> Result<Statement> {
        self.advance(); // Skip GIMMEH

        let target = if let Some(token) = self.current() {
            if let TokenType::Identifier(name) = &token.token_type {
                let n = name.clone();
                self.advance();
                n
            } else {
                return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
            }
        } else {
            return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
        };

        self.skip_newlines();
        Ok(Statement::Input { target })
    }

    fn parse_if_then_else(&mut self) -> Result<Statement> {
        self.advance(); // Skip O RLY?
        self.skip_newlines();

        let condition = None; // Uses implicit variable IT

        // Parse YA RLY block
        self.expect(&TokenType::YaRly)?;
        self.skip_newlines();

        let mut then_block = Vec::new();
        while let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Mebbe | TokenType::NoWai | TokenType::Oic) {
                break;
            }
            then_block.push(self.parse_statement()?);
        }

        // Parse MEBBE blocks (else-if)
        let mut else_ifs = Vec::new();
        while let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Mebbe) {
                self.advance();
                let cond = self.parse_expression()?;
                self.skip_newlines();

                let mut block = Vec::new();
                while let Some(t) = self.current() {
                    if matches!(t.token_type, TokenType::Mebbe | TokenType::NoWai | TokenType::Oic) {
                        break;
                    }
                    block.push(self.parse_statement()?);
                }
                else_ifs.push((cond, block));
            } else {
                break;
            }
        }

        // Parse NO WAI block (else)
        let else_block = if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::NoWai) {
                self.advance();
                self.skip_newlines();

                let mut block = Vec::new();
                while let Some(t) = self.current() {
                    if matches!(t.token_type, TokenType::Oic) {
                        break;
                    }
                    block.push(self.parse_statement()?);
                }
                Some(block)
            } else {
                None
            }
        } else {
            None
        };

        self.expect(&TokenType::Oic)?;
        self.skip_newlines();

        Ok(Statement::IfThenElse {
            condition,
            then_block,
            else_ifs,
            else_block,
        })
    }

    fn parse_loop(&mut self) -> Result<Statement> {
        self.advance(); // Skip IM IN YR
        
        let label = if let Some(token) = self.current() {
            if let TokenType::Identifier(name) = &token.token_type {
                let n = name.clone();
                self.advance();
                n
            } else {
                return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
            }
        } else {
            return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
        };

        // Parse operation and variable (optional)
        let (operation, variable) = if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Uppin | TokenType::Nerfin) {
                let op = if matches!(token.token_type, TokenType::Uppin) {
                    "UPPIN"
                } else {
                    "NERFIN"
                }.to_string();
                self.advance();
                
                self.expect(&TokenType::Yr)?;
                
                let var = if let Some(t) = self.current() {
                    if let TokenType::Identifier(name) = &t.token_type {
                        let v = name.clone();
                        self.advance();
                        Some(v)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                (Some(op), var)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        // Parse condition (optional)
        let condition = if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Til | TokenType::Wile) {
                let cond_type = if matches!(token.token_type, TokenType::Til) {
                    "TIL"
                } else {
                    "WILE"
                }.to_string();
                self.advance();
                
                let expr = self.parse_expression()?;
                Some((cond_type, expr))
            } else {
                None
            }
        } else {
            None
        };

        self.skip_newlines();

        // Parse body
        let mut body = Vec::new();
        while let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::ImOuttaYr) {
                break;
            }
            body.push(self.parse_statement()?);
        }

        self.expect(&TokenType::ImOuttaYr)?;
        // Skip loop label verification for simplicity
        if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Identifier(_)) {
                self.advance();
            }
        }
        self.skip_newlines();

        Ok(Statement::Loop {
            label,
            operation,
            variable,
            condition,
            body,
        })
    }

    fn parse_function_def(&mut self) -> Result<Statement> {
        self.advance(); // Skip HOW IZ
        
        // Skip "I" (function scope)
        if let Some(token) = self.current() {
            if let TokenType::Identifier(name) = &token.token_type {
                if name == "I" {
                    self.advance();
                }
            }
        }

        let name = if let Some(token) = self.current() {
            if let TokenType::Identifier(n) = &token.token_type {
                let name = n.clone();
                self.advance();
                name
            } else {
                return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
            }
        } else {
            return Err(LciError::ExpectedIdentifier(String::new(), 0, String::new()));
        };

        // Parse parameters
        let mut params = Vec::new();
        if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::Yr) {
                self.advance();
                
                loop {
                    if let Some(t) = self.current() {
                        if let TokenType::Identifier(param) = &t.token_type {
                            params.push(param.clone());
                            self.advance();
                            
                            // Check for AN YR for more params
                            if let Some(next) = self.current() {
                                if matches!(next.token_type, TokenType::AnYr) {
                                    self.advance();
                                    continue;
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }

        self.skip_newlines();

        // Parse body
        let mut body = Vec::new();
        while let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::IfUSaySo) {
                break;
            }
            body.push(self.parse_statement()?);
        }

        self.expect(&TokenType::IfUSaySo)?;
        self.skip_newlines();

        Ok(Statement::FunctionDef { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Statement> {
        self.advance(); // Skip FOUND YR

        let value = if let Some(token) = self.current() {
            if !matches!(token.token_type, TokenType::Newline | TokenType::Eof) {
                Some(self.parse_expression()?)
            } else {
                None
            }
        } else {
            None
        };

        self.skip_newlines();
        Ok(Statement::Return { value })
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        if let Some(token) = self.current() {
            match &token.token_type {
                TokenType::Integer(val) => {
                    let v = *val;
                    self.advance();
                    Ok(Expression::Integer(v))
                }
                TokenType::Float(val) => {
                    let v = *val;
                    self.advance();
                    Ok(Expression::Float(v))
                }
                TokenType::String(val) => {
                    let v = val.clone();
                    self.advance();
                    Ok(Expression::String(v))
                }
                TokenType::Boolean(val) => {
                    let v = *val;
                    self.advance();
                    Ok(Expression::Boolean(v))
                }
                TokenType::Noob => {
                    self.advance();
                    Ok(Expression::Nil)
                }
                TokenType::It => {
                    self.advance();
                    Ok(Expression::ImplicitVariable)
                }
                TokenType::Identifier(name) => {
                    let n = name.clone();
                    self.advance();
                    Ok(Expression::Variable(n))
                }
                // Binary operators
                TokenType::SumOf => self.parse_binary_op(BinaryOperator::Add),
                TokenType::DiffOf => self.parse_binary_op(BinaryOperator::Sub),
                TokenType::ProduktOf => self.parse_binary_op(BinaryOperator::Mul),
                TokenType::QuoshuntOf => self.parse_binary_op(BinaryOperator::Div),
                TokenType::ModOf => self.parse_binary_op(BinaryOperator::Mod),
                TokenType::BiggrOf => self.parse_binary_op(BinaryOperator::Max),
                TokenType::SmallrOf => self.parse_binary_op(BinaryOperator::Min),
                TokenType::BothOf => self.parse_binary_op(BinaryOperator::And),
                TokenType::EitherOf => self.parse_binary_op(BinaryOperator::Or),
                TokenType::WonOf => self.parse_binary_op(BinaryOperator::Xor),
                TokenType::BothSaem => self.parse_binary_op(BinaryOperator::Equal),
                TokenType::Diffrint => self.parse_binary_op(BinaryOperator::NotEqual),
                // Unary operators
                TokenType::Not => self.parse_unary_op(UnaryOperator::Not),
                _ => Err(LciError::ExpectedExpression(String::new(), 0, String::new())),
            }
        } else {
            Err(LciError::ExpectedExpression(String::new(), 0, String::new()))
        }
    }

    fn parse_binary_op(&mut self, op: BinaryOperator) -> Result<Expression> {
        self.advance(); // Skip operator
        let left = Box::new(self.parse_expression()?);
        
        // Skip AN separator if present
        if let Some(token) = self.current() {
            if matches!(token.token_type, TokenType::An) {
                self.advance();
            }
        }
        
        let right = Box::new(self.parse_expression()?);
        
        Ok(Expression::BinaryOp { op, left, right })
    }

    fn parse_unary_op(&mut self, op: UnaryOperator) -> Result<Expression> {
        self.advance(); // Skip operator
        let operand = Box::new(self.parse_expression()?);
        Ok(Expression::UnaryOp { op, operand })
    }
}
