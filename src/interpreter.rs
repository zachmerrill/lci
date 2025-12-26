//! Interpreter module for the LCI LOLCODE interpreter.
//!
//! The interpreter executes the AST produced by the parser.

use crate::error::{LciError, Result};
use crate::parser::*;
use std::collections::HashMap;
use std::io::{self, Write};

/// Runtime value types
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f32),
    String(String),
    Boolean(bool),
    Nil,
    Function {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

impl Value {
    /// Convert value to boolean
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
            Value::Function { .. } => true,
        }
    }

    /// Convert value to integer
    pub fn to_integer(&self) -> Result<i64> {
        match self {
            Value::Integer(i) => Ok(*i),
            Value::Float(f) => Ok(*f as i64),
            Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
            Value::String(s) => s.parse().map_err(|_| LciError::ExpectedIntegerValue),
            Value::Nil => Err(LciError::CannotImplicitlyCastNil),
            Value::Function { .. } => Err(LciError::CannotCastFunctionToInteger),
        }
    }

    /// Convert value to float
    pub fn to_float(&self) -> Result<f32> {
        match self {
            Value::Integer(i) => Ok(*i as f32),
            Value::Float(f) => Ok(*f),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => s.parse().map_err(|_| LciError::ExpectedDecimal),
            Value::Nil => Err(LciError::CannotImplicitlyCastNil),
            Value::Function { .. } => Err(LciError::CannotCastFunctionToDecimal),
        }
    }

    /// Convert value to string
    pub fn to_string(&self) -> Result<String> {
        match self {
            Value::Integer(i) => Ok(i.to_string()),
            Value::Float(f) => Ok(f.to_string()),
            Value::Boolean(b) => Ok(if *b { "WIN" } else { "FAIL" }.to_string()),
            Value::String(s) => Ok(s.clone()),
            Value::Nil => Ok(String::new()),
            Value::Function { .. } => Err(LciError::CannotCastFunctionToString),
        }
    }
}

/// Return status from statement execution
#[derive(Debug, Clone)]
pub enum ReturnStatus {
    Normal,
    Break,
    Return(Value),
}

/// Interpreter execution context
pub struct Interpreter {
    /// Variable scopes (stack of hash maps)
    scopes: Vec<HashMap<String, Value>>,
    /// Implicit variable (IT)
    implicit_var: Value,
    /// Functions defined in global scope
    functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            implicit_var: Value::Nil,
            functions: HashMap::new(),
        }
    }

    /// Push a new variable scope
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current variable scope
    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Get a variable from the current scope or parent scopes
    fn get_variable(&self, name: &str) -> Result<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(LciError::VariableDoesNotExist(
            String::new(),
            0,
            name.to_string(),
        ))
    }

    /// Set a variable in the current scope
    fn set_variable(&mut self, name: String, value: Value) -> Result<()> {
        // Try to find variable in existing scopes
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return Ok(());
            }
        }
        
        // If not found, create in current scope
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
            Ok(())
        } else {
            Err(LciError::UnableToStoreVariable(
                String::new(),
                0,
                name,
            ))
        }
    }

    /// Declare a new variable in the current scope
    fn declare_variable(&mut self, name: String, value: Option<Value>) -> Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(LciError::RedefinitionOfVariable(
                    String::new(),
                    0,
                    name,
                ));
            }
            scope.insert(name, value.unwrap_or(Value::Nil));
            Ok(())
        } else {
            Err(LciError::UnableToStoreVariable(
                String::new(),
                0,
                name,
            ))
        }
    }

    /// Interpret the main program node
    pub fn interpret(&mut self, program: &MainNode) -> Result<()> {
        for statement in &program.statements {
            let status = self.execute_statement(statement)?;
            if !matches!(status, ReturnStatus::Normal) {
                break;
            }
        }
        Ok(())
    }

    /// Execute a statement
    fn execute_statement(&mut self, stmt: &Statement) -> Result<ReturnStatus> {
        match stmt {
            Statement::Declaration { name, value } => {
                let val = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Nil
                };
                self.declare_variable(name.clone(), Some(val))?;
                Ok(ReturnStatus::Normal)
            }
            
            Statement::Assignment { target, value } => {
                let val = self.evaluate_expression(value)?;
                self.set_variable(target.clone(), val)?;
                Ok(ReturnStatus::Normal)
            }
            
            Statement::Print {
                expressions,
                newline,
                to_stderr,
            } => {
                let output: Result<Vec<String>> = expressions
                    .iter()
                    .map(|e| {
                        let val = self.evaluate_expression(e)?;
                        val.to_string()
                    })
                    .collect();
                
                let text = output?.join(" ");
                
                if *to_stderr {
                    eprint!("{}", text);
                    if *newline {
                        eprintln!();
                    }
                    let _ = io::stderr().flush();
                } else {
                    print!("{}", text);
                    if *newline {
                        println!();
                    }
                    let _ = io::stdout().flush();
                }
                
                Ok(ReturnStatus::Normal)
            }
            
            Statement::Input { target } => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input = input.trim().to_string();
                
                // Try to parse as different types
                let value = if let Ok(i) = input.parse::<i64>() {
                    Value::Integer(i)
                } else if let Ok(f) = input.parse::<f32>() {
                    Value::Float(f)
                } else if input == "WIN" {
                    Value::Boolean(true)
                } else if input == "FAIL" {
                    Value::Boolean(false)
                } else {
                    Value::String(input)
                };
                
                self.set_variable(target.clone(), value)?;
                Ok(ReturnStatus::Normal)
            }
            
            Statement::IfThenElse {
                condition,
                then_block,
                else_ifs,
                else_block,
            } => {
                let cond_value = if let Some(expr) = condition {
                    self.evaluate_expression(expr)?
                } else {
                    self.implicit_var.clone()
                };
                
                if cond_value.to_bool() {
                    self.push_scope();
                    for stmt in then_block {
                        let status = self.execute_statement(stmt)?;
                        if !matches!(status, ReturnStatus::Normal) {
                            self.pop_scope();
                            return Ok(status);
                        }
                    }
                    self.pop_scope();
                } else {
                    // Try else-if branches
                    let mut executed = false;
                    for (cond_expr, block) in else_ifs {
                        let cond = self.evaluate_expression(cond_expr)?;
                        if cond.to_bool() {
                            self.push_scope();
                            for stmt in block {
                                let status = self.execute_statement(stmt)?;
                                if !matches!(status, ReturnStatus::Normal) {
                                    self.pop_scope();
                                    return Ok(status);
                                }
                            }
                            self.pop_scope();
                            executed = true;
                            break;
                        }
                    }
                    
                    // Execute else block if no else-if matched
                    if !executed {
                        if let Some(block) = else_block {
                            self.push_scope();
                            for stmt in block {
                                let status = self.execute_statement(stmt)?;
                                if !matches!(status, ReturnStatus::Normal) {
                                    self.pop_scope();
                                    return Ok(status);
                                }
                            }
                            self.pop_scope();
                        }
                    }
                }
                
                Ok(ReturnStatus::Normal)
            }
            
            Statement::Loop {
                label: _,
                operation,
                variable,
                condition,
                body,
            } => {
                self.push_scope();
                
                loop {
                    // Check condition
                    if let Some((cond_type, cond_expr)) = condition {
                        let cond_val = self.evaluate_expression(cond_expr)?;
                        let should_continue = cond_val.to_bool();
                        
                        if cond_type == "TIL" && should_continue {
                            break;
                        } else if cond_type == "WILE" && !should_continue {
                            break;
                        }
                    }
                    
                    // Execute body
                    for stmt in body {
                        match self.execute_statement(stmt)? {
                            ReturnStatus::Break => {
                                self.pop_scope();
                                return Ok(ReturnStatus::Normal);
                            }
                            ReturnStatus::Return(val) => {
                                self.pop_scope();
                                return Ok(ReturnStatus::Return(val));
                            }
                            ReturnStatus::Normal => {}
                        }
                    }
                    
                    // Perform operation
                    if let (Some(op), Some(var)) = (operation, variable) {
                        let current = self.get_variable(var)?;
                        let new_value = if op == "UPPIN" {
                            let i = current.to_integer()?;
                            Value::Integer(i + 1)
                        } else {
                            let i = current.to_integer()?;
                            Value::Integer(i - 1)
                        };
                        self.set_variable(var.clone(), new_value)?;
                    }
                }
                
                self.pop_scope();
                Ok(ReturnStatus::Normal)
            }
            
            Statement::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(ReturnStatus::Normal)
            }
            
            Statement::Return { value } => {
                let val = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Nil
                };
                Ok(ReturnStatus::Return(val))
            }
            
            Statement::Break => Ok(ReturnStatus::Break),
            
            Statement::Expression(expr) => {
                let val = self.evaluate_expression(expr)?;
                self.implicit_var = val;
                Ok(ReturnStatus::Normal)
            }
            
            Statement::FunctionCall { name, args } => {
                let _ = self.call_function(name, args)?;
                Ok(ReturnStatus::Normal)
            }
        }
    }

    /// Evaluate an expression
    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Integer(val) => Ok(Value::Integer(*val)),
            Expression::Float(val) => Ok(Value::Float(*val)),
            Expression::String(val) => Ok(Value::String(val.clone())),
            Expression::Boolean(val) => Ok(Value::Boolean(*val)),
            Expression::Nil => Ok(Value::Nil),
            Expression::Variable(name) => self.get_variable(name),
            Expression::ImplicitVariable => Ok(self.implicit_var.clone()),
            
            Expression::BinaryOp { op, left, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_op(*op, left_val, right_val)
            }
            
            Expression::UnaryOp { op, operand } => {
                let val = self.evaluate_expression(operand)?;
                self.apply_unary_op(*op, val)
            }
            
            Expression::FunctionCall { name, args } => {
                self.call_function(name, args)
            }
            
            Expression::Cast { expr, target_type } => {
                let val = self.evaluate_expression(expr)?;
                self.cast_value(val, *target_type)
            }
        }
    }

    /// Apply a binary operator
    fn apply_binary_op(&self, op: BinaryOperator, left: Value, right: Value) -> Result<Value> {
        match op {
            BinaryOperator::Add => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                Ok(Value::Integer(l + r))
            }
            BinaryOperator::Sub => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                Ok(Value::Integer(l - r))
            }
            BinaryOperator::Mul => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                Ok(Value::Integer(l * r))
            }
            BinaryOperator::Div => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                if r == 0 {
                    return Err(LciError::DivisionByZero);
                }
                Ok(Value::Integer(l / r))
            }
            BinaryOperator::Mod => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                if r == 0 {
                    return Err(LciError::DivisionByZero);
                }
                Ok(Value::Integer(l % r))
            }
            BinaryOperator::Max => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                Ok(Value::Integer(l.max(r)))
            }
            BinaryOperator::Min => {
                let l = left.to_integer()?;
                let r = right.to_integer()?;
                Ok(Value::Integer(l.min(r)))
            }
            BinaryOperator::And => Ok(Value::Boolean(left.to_bool() && right.to_bool())),
            BinaryOperator::Or => Ok(Value::Boolean(left.to_bool() || right.to_bool())),
            BinaryOperator::Xor => Ok(Value::Boolean(left.to_bool() ^ right.to_bool())),
            BinaryOperator::Equal => {
                // Simple equality check
                let equal = match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => a == b,
                    (Value::Float(a), Value::Float(b)) => a == b,
                    (Value::Boolean(a), Value::Boolean(b)) => a == b,
                    (Value::String(a), Value::String(b)) => a == b,
                    (Value::Nil, Value::Nil) => true,
                    _ => false,
                };
                Ok(Value::Boolean(equal))
            }
            BinaryOperator::NotEqual => {
                let equal = match (&left, &right) {
                    (Value::Integer(a), Value::Integer(b)) => a == b,
                    (Value::Float(a), Value::Float(b)) => a == b,
                    (Value::Boolean(a), Value::Boolean(b)) => a == b,
                    (Value::String(a), Value::String(b)) => a == b,
                    (Value::Nil, Value::Nil) => true,
                    _ => false,
                };
                Ok(Value::Boolean(!equal))
            }
        }
    }

    /// Apply a unary operator
    fn apply_unary_op(&self, op: UnaryOperator, val: Value) -> Result<Value> {
        match op {
            UnaryOperator::Not => Ok(Value::Boolean(!val.to_bool())),
        }
    }

    /// Call a function
    fn call_function(&mut self, name: &str, args: &[Expression]) -> Result<Value> {
        // Evaluate arguments
        let arg_values: Result<Vec<Value>> = args
            .iter()
            .map(|e| self.evaluate_expression(e))
            .collect();
        let arg_values = arg_values?;

        // Look up function
        if let Some((params, body)) = self.functions.get(name).cloned() {
            if params.len() != arg_values.len() {
                return Err(LciError::IncorrectNumberOfArguments(
                    String::new(),
                    0,
                    name.to_string(),
                ));
            }

            // Create new scope with parameters
            self.push_scope();
            for (param, value) in params.iter().zip(arg_values.iter()) {
                self.declare_variable(param.clone(), Some(value.clone()))?;
            }

            // Execute function body
            let mut return_value = Value::Nil;
            for stmt in &body {
                match self.execute_statement(stmt)? {
                    ReturnStatus::Return(val) => {
                        return_value = val;
                        break;
                    }
                    ReturnStatus::Break => {
                        break;
                    }
                    ReturnStatus::Normal => {}
                }
            }

            self.pop_scope();
            Ok(return_value)
        } else {
            Err(LciError::UndefinedFunction(
                String::new(),
                0,
                name.to_string(),
            ))
        }
    }

    /// Cast a value to a target type
    fn cast_value(&self, val: Value, target: ValueType) -> Result<Value> {
        match target {
            ValueType::Noob => Ok(Value::Nil),
            ValueType::Troof => Ok(Value::Boolean(val.to_bool())),
            ValueType::Numbr => Ok(Value::Integer(val.to_integer()?)),
            ValueType::Numbar => Ok(Value::Float(val.to_float()?)),
            ValueType::Yarn => Ok(Value::String(val.to_string()?)),
            ValueType::Bukkit => Err(LciError::CannotCastValueToArray(String::new(), 0, String::new())),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
