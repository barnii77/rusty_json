/* use std::str::FromStr;

pub enum JsonParseError {
    /// Not a valid token
    InvalidSyntax(String),
    /// Not a valid sequence of tokens
    UnexpectedToken(String),
}


enum Constant {
    Float(f64),
    Int(isize),
    StringLiteral(String),
    Null,
}

impl FromStr for Constant {
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_string();
        if s.starts_with('"') || s.ends_with('"') {
            if !s.starts_with('"') || !s.starts_with('"') {
                Err(Self::Err::InvalidSyntax(s))
            } else {
                Ok(Self::StringLiteral(s))
            }
        } else if s == "null" {
            Ok(Self::Null)
        } else if let Ok(integer) = s.parse::<isize>() {
            Ok(Self::Int(integer))
        } else if let Ok(float) = s.parse::<f64>() {
            Ok(Self::Float(float))
        } else {
            Err(Self::Err::InvalidSyntax(s))
        }
    }
}

enum Token {
    // StartOfSequence,
    StartOfDict,
    StartOfList,
    EndOfDict,
    EndOfList,
    Constant(Constant)
}

impl FromStr for Token {
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            if !s.ends_with('"') {
                return Err(Self::Err::InvalidSyntax(s.to_string()))
            }
            Ok(Self::Constant(Constant::StringLiteral(s.to_string())))
        } else if s == "{" {
            Ok(Self::StartOfDict)
        } else if s == "}" {
            Ok(Self::EndOfDict)
        } else if s == "[" {
            Ok(Self::StartOfList)
        } else if s == "]" {
            Ok(Self::EndOfList)
        } else {
            Ok(Self::Constant(Constant::from_str(s)?))
        }
    }
}

enum ParseState {
    Start,
    Dict,
    List,
    Key,
    Value,
}

fn parse(json: String) {
    let mut tokens: Vec<Token> = vec![];
    let mut state = ParseState::Start;
    let mut buffer = String::new();
    for c in json.chars() {
        if state.allows(c) {
            buffer.push(c);
        } else if state.terminated_by(c) {
            tokens.push(Token::from_str(&buffer)?);
        } else {
            Token::from_str(&buffer).unwrap_err()
        }
    }
} */

pub mod lexer; 
pub mod parser;
