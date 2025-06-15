use crate::utils::parser::parseExp;

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn from_char (c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            _ => None,
        }
    }
    pub fn from_str (s: &String) -> Option<Operator> {
        match s.as_str() {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            _ => None,
        }
    }
}

pub fn lexer (e: &String) {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer: String = String::from("");

    for c in e.chars() {
        if c.is_digit(10) {
            buffer.push(c);
        } else if let Some(op) = Operator::from_char(c) {
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                buffer.clear();
            }
            tokens.push(c.to_string());
        }
    }

    if buffer.len() != 0 {
        tokens.push(buffer);
    }

    println!("\nTokens: {:?}", tokens);
    parseExp(tokens);
}
