use std::collections::HashMap;
use std::io::{self, Write};

/// Define binary operations
enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Define expressions
enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
    Let(String, Box<Expr>),
}

/// Predefined functions
type Func = fn(f64, f64) -> f64;

fn sqrt(a: f64, _: f64) -> f64 {
    a.sqrt()
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

/// Function to return predefined functions
fn predefined_functions() -> HashMap<String, Func> {
    let mut funcs = HashMap::new();
    funcs.insert("sqrt".to_string(), sqrt as Func);
    funcs.insert("max".to_string(), max as Func);
    funcs
}

/// Context to store variables and functions
struct Context {
    variables: HashMap<String, f64>,
    functions: HashMap<String, Func>,
}

impl Context {
    fn new() -> Self {
        Context {
            variables: HashMap::new(),
            functions: predefined_functions(),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Variable(name) => *self.variables.get(name).unwrap_or(&0.0),
            Expr::BinaryOp(lhs, op, rhs) => {
                let left = self.eval_expr(lhs);
                let right = self.eval_expr(rhs);
                match op {
                    BinOp::Add => left + right,
                    BinOp::Subtract => left - right,
                    BinOp::Multiply => left * right,
                    BinOp::Divide => left / right,
                }
            }
            Expr::FunctionCall(name, args) => {
                let func = self.functions.get(name).unwrap().clone();
                let arg1 = self.eval_expr(&args[0]);
                let arg2 = if args.len() > 1 {
                    self.eval_expr(&args[1])
                } else {
                    0.0
                };
                func(arg1, arg2)
            }
            Expr::Let(name, value_expr) => {
                let value = self.eval_expr(value_expr);
                self.set_variable(name.clone(), value);
                value
            }
        }
    }

    fn set_variable(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }
}

/// Simple tokenizer to break down input into tokens
#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Identifier(String),
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    OpenParen,
    CloseParen,
    Comma,
    Semicolon,
    Eof,
}

/// Tokenizer function (basic version, can be extended)
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Assign);
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_numeric() || ch == '.' {
                        num.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse::<f64>().unwrap()));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if ident == "let" {
                    tokens.push(Token::Let);
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            }
            _ => {
                chars.next(); // Ignore unrecognized characters
            }
        }
    }
    tokens.push(Token::Eof);
    tokens
}

/// Parser to convert tokens into expressions
fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.into_iter().peekable();
    parse_expr(&mut tokens)
}

fn parse_expr(tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Expr {
    if let Some(token) = tokens.next() {
        match token {
            Token::Let => {
                if let Some(Token::Identifier(var)) = tokens.next() {
                    if let Some(Token::Assign) = tokens.next() {
                        let expr = parse_expr(tokens);
                        return Expr::Let(var, Box::new(expr));
                    }
                }
            }
            Token::Number(n) => {
                return Expr::Number(n);
            }
            Token::Identifier(name) => {
                if let Some(Token::OpenParen) = tokens.peek() {
                    tokens.next(); // consume '('
                    let mut args = Vec::new();
                    while let Some(token) = tokens.peek() {
                        if *token == Token::CloseParen {
                            break;
                        }
                        args.push(parse_expr(tokens));
                        if tokens.peek() == Some(&Token::Comma) {
                            tokens.next(); // consume ','
                        }
                    }
                    tokens.next(); // consume ')'
                    return Expr::FunctionCall(name, args);
                }
                return Expr::Variable(name);
            }
            _ => {}
        }
    }
    Expr::Number(0.0) // Default case
}

/// REPL loop
fn repl() {
    let mut context = Context::new();

    loop {
        // Read user input
        let mut input = String::new();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Tokenize and parse the input
        let tokens = tokenize(&input);
        let expr = parse(tokens);

        // Evaluate the expression
        let result = context.eval_expr(&expr);
        println!("=> {}", result);
    }
}

fn main() {
    repl();
}
