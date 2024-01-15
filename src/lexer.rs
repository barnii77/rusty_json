use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonLexError {
    /// Not a valid token
    #[error("Invalid syntax in token: {0}")]
    InvalidSyntax(String),
}

#[derive(Debug, Clone)]
pub enum Constant {
    Float(f64),
    Int(isize),
    StringLiteral(String),
    Null,
    Boolean(bool),
}

impl FromStr for Constant {
    type Err = JsonLexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim(); // this to_string could be avoided in some cases
        if s.starts_with('"') || s.ends_with('"') {
            if !s.starts_with('"') || !s.starts_with('"') || s.len() < 2 {
                // 2 seperate quotes
                Err(Self::Err::InvalidSyntax(s.to_string()))
            } else {
                Ok(Self::StringLiteral(s[1..s.len() - 1].to_string())) // cut off quotes
            }
        } else if s == "null" {
            Ok(Self::Null)
        } else if let Ok(integer) = s.parse::<isize>() {
            Ok(Self::Int(integer))
        } else if let Ok(float) = s.parse::<f64>() {
            Ok(Self::Float(float))
        } else if s == "true" || s == "false" {
            Ok(Self::Boolean(s == "true"))
        } else {
            Err(Self::Err::InvalidSyntax(s.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Token {
    // StartOfSequence,
    StartOfDict,
    StartOfList,
    EndOfDict,
    EndOfList,
    Constant(Constant),
    Colon,
    Comma,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Token {
    type Err = JsonLexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "{" => Ok(Self::StartOfDict),
            "}" => Ok(Self::EndOfDict),
            "[" => Ok(Self::StartOfList),
            "]" => Ok(Self::EndOfList),
            ":" => Ok(Self::Colon),
            "," => Ok(Self::Comma),
            _ => Ok(Self::Constant(Constant::from_str(s)?)),
        }
    }
}

#[derive(PartialEq)]
enum LexStateType {
    // TODO add True and False states
    Any,
    Numeric,
    Int,
    Float,
    String,
    Null,
    Invalid,
    Symbol, // Includes { } [ ] , : ... those are all single character, meaning any character
            // terminates them.
}

struct LexState {
    state: LexStateType,
    buffer: String,
}

impl LexState {
    fn is_any(&self) -> bool {
        self.state == LexStateType::Any
    }

    fn is_invalid(&self) -> bool {
        self.state == LexStateType::Invalid
    }

    fn ignores(&self, c: char) -> bool {
        self.state == LexStateType::Any && (c == ' ' || c == '\n' || c == '\t')
    }

    fn new() -> Self {
        Self {
            state: LexStateType::Any,
            buffer: String::new(),
        }
    }

    fn encorporate(&mut self, c: char) {
        if self.ignores(c) {
            return;
        }
        self.buffer.push(c);
        match self.state {
            LexStateType::Any => {
                if c.is_numeric() {
                    self.state = LexStateType::Numeric;
                } else if c == '.' {
                    self.state = LexStateType::Float;
                } else if c == '"' {
                    self.state = LexStateType::String;
                } else if c == 'n' {
                    self.state = LexStateType::Null;
                } else if c == ' ' || c == '\n' || c == '\t' {
                    self.state = LexStateType::Any;
                } else if c == '{' || c == '}' || c == '[' || c == ']' || c == ':' || c == ',' {
                    self.state = LexStateType::Symbol;
                } else {
                    self.state = LexStateType::Invalid;
                }
            }
            LexStateType::Numeric => {
                if c == '.' {
                    self.state = LexStateType::Float;
                } else if c.is_numeric() {
                    self.state = LexStateType::Numeric;
                } else {
                    self.state = LexStateType::Invalid;
                }
            }
            LexStateType::Int => {
                if c.is_numeric() {
                    self.state = LexStateType::Int;
                } else {
                    self.state = LexStateType::Invalid;
                }
            }
            LexStateType::Float => {
                if c.is_numeric() {
                    self.state = LexStateType::Float;
                } else if c == '.' {
                    let prev_option = self.buffer.chars().last();
                    if let Some(prev) = prev_option {
                        if prev == '.' {
                            self.state = LexStateType::Invalid;
                        } else {
                            self.state = LexStateType::Float;
                        }
                    } else {
                        // Self { state: LexStateType::Invalid, buffer }
                        panic!("State had type Float even though buffer was empty, so there was no way to infer this information.")
                    }
                } else {
                    self.state = LexStateType::Invalid;
                }
            }
            LexStateType::String => {
                self.state = LexStateType::String;
            }
            LexStateType::Null => {
                if self.buffer.is_empty() {
                    panic!("State had type Null even though self.buffer was empty, so there was no way to infer this information.")
                } else if self.buffer == "null"
                    || self.buffer == "nul"
                    || self.buffer == "nu"
                    || self.buffer == "n"
                {
                    self.state = LexStateType::Null;
                } else {
                    self.state = LexStateType::Invalid;
                }
            }
            LexStateType::Invalid => {
                panic!("Program kept lexing even though an invalid state was reached.")
            } // Consider panic, you shouldnt keep going once
            // you reached an invalid state, so this should never be reached if the program
            // functions correctly.
            LexStateType::Symbol => {
                panic!("Symbols are single character, so in the case that one occurs, after the symbol LexStateType instance had been created, in the next parsing step, anything follwing the symbol should have been marked as terminal and thus the Symbol should have ended. This is unreachable in a functioning program.")
            }
        }
    }

    fn is_terminated_by(&self, c: char) -> bool {
        match self.state {
            LexStateType::Any => false,
            LexStateType::Numeric => !(c.is_numeric() || c == '.'),
            LexStateType::Int => !c.is_numeric(),
            LexStateType::Float => !c.is_numeric(),
            LexStateType::String => {
                // The following checks if the previous character is " &
                // before it is not a backslash
                self.buffer
                    .chars()
                    .last()
                    .expect("Must not be empty because it is in state String")
                    == '"'
                    && self
                        .buffer
                        .chars()
                        .take(self.buffer.len() - 1)
                        .last()
                        .unwrap_or(' ')
                        != '\\'
            }
            LexStateType::Null => match self.buffer.as_str() {
                "null" => c != ' ',
                "nul" => c != 'l',
                "nu" => c != 'l',
                "n" => c != 'u',
                _ => true,
            },
            LexStateType::Invalid => {
                panic!("Invalid state reached, so this should never be called.")
            }
            LexStateType::Symbol => true,
        }
    }

    fn allows(&self, c: char) -> bool {
        match self.state {
            LexStateType::Any => true,
            LexStateType::Numeric => c.is_numeric() || c == '.',
            LexStateType::Int => c.is_numeric(),
            LexStateType::Float => c.is_numeric(),
            LexStateType::String => {
                self.buffer
                    .chars()
                    .take(self.buffer.len() - 1)
                    .last()
                    .unwrap_or('\\')  // unwrap_or('\\') so that if there is no character before it
                // (meaning we are checking the string opening quotes), it should not terminate
                    == '\\'
                    || self
                        .buffer
                        .chars()
                        .last()
                        .expect("Must not be empty because of state String")
                        != '"'
            } // string allows any character apart from " (except if there is a \ before it)
            LexStateType::Null => match self.buffer.as_str() {
                "null" => c == ' ',
                "nul" => c == 'l',
                "nu" => c == 'l',
                "n" => c == 'u',
                _ => false,
            },
            LexStateType::Invalid => {
                panic!("Invalid state reached, so this should never be called.")
            }
            LexStateType::Symbol => false,
        }
    }
}

pub(crate) struct Tokenizer<'a> {
    json: std::iter::Peekable<std::str::Chars<'a>>,
    state: LexState,
}

impl<'a> Tokenizer<'a> {
    fn new(json: &'a str) -> Self {
        Self {
            json: json.chars().peekable(),
            state: LexState::new(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, JsonLexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_invalid() {
            return None;
        }
        for c in self.json.by_ref() {
            if self.state.allows(c) {
                self.state.encorporate(c);
            } else if self.state.is_terminated_by(c) {
                let token_result = Token::from_str(&self.state.buffer);
                if token_result.is_err() {
                    self.state = LexState {
                        state: LexStateType::Invalid,
                        buffer: c.to_string(),
                    };
                } else {
                    self.state = LexState::new();
                    self.state.encorporate(c);
                }
                return Some(token_result);
            } else {
                // not allowed + not terminated by -> syntax error
                self.state.buffer.push(c); // push the invalid character so that the conversion to token will
                                           // fail with appropriate error.
                return Some(Err(Token::from_str(&self.state.buffer).unwrap_err()));
            }
        }
        if !self.state.is_any() {
            let result = Some(Token::from_str(&self.state.buffer));
            self.state = LexState::new();
            result
        } else {
            None
        }
    }
}

// not actually needed (because of Tokenizer), but won't delete it either.
pub fn tokenize(json: &str) -> Result<Vec<Token>, JsonLexError> {
    let mut tokens: Vec<Token> = vec![];
    let mut state = LexState::new();
    for c in json.chars() {
        if state.allows(c) {
            state.encorporate(c);
        } else if state.is_terminated_by(c) {
            tokens.push(Token::from_str(&state.buffer)?);
            state = LexState::new();
            state.encorporate(c);
        } else {
            state.buffer.push(c); // push the invalid character so that the conversion to token will
                                  // fail with appropriate error.
            return Err(Token::from_str(&state.buffer).unwrap_err());
        }
    }
    if state.state != LexStateType::Any {
        tokens.push(Token::from_str(&state.buffer)?);
    }
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize_1() {
        /* let json = r#"
        {
            "key1": 1,
            "key2": "value2",
            "key3": [1, 2, 3],
            "key4": {
                "key5": "value5",
                "key6": 6
            }
        }
        "#; */
        let json = r#"{"key": "value"}"#;
        let tokens = tokenize(json).expect("This should not crash");
        println!("**Tokens**: {:?}", tokens);
    }

    #[test]
    fn test_tokenize_2() {
        let json = r#"
        {
            "key1": 1,
            "key2": "value2",
            "key3": [1, 2, 3],
            "key4": {
                "key5": "value5",
                "key6": 6
            }
        }
        "#;
        let tokens = tokenize(json).expect("This should not crash");
        println!("**Tokens**: {:?}", tokens);
    }

    #[test]
    fn test_tokenizer_1() {
        let json = r#"{"key": "value"}"#;
        let mut tokenizer = Tokenizer::new(json);
        while let Some(token) = tokenizer.next() {
            println!("Token: {:?}", token);
        }
    }

    #[test]
    fn test_tokenizer_2() {
        let json = r#"
        {
            "key1": 1,
            "key2": "value2",
            "key3": [1, 2, 3],
            "key4": {
                "key5": "value5",
                "key6": 6
            }
        }
        "#;
        let mut tokenizer = Tokenizer::new(json);
        while let Some(token) = tokenizer.next() {
            println!("Token: {:?}", token);
        }
    }
}
