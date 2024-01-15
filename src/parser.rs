use crate::lexer::{tokenize, Constant, JsonLexError, Token, Tokenizer};
use std::collections::HashMap;
use thiserror::Error;

// TODO create a datastructure that can store the json
// TODO create a parser that can parse the tokenized json to this datastructure

#[derive(Debug, Error)]
pub enum JsonParseError {
    #[error("JsonParseError: Invalid token: {0}")]
    UnexpectedToken(Token),
    #[error("JsonParseError: Unexpected end of input")]
    UnexpectedEndOfInput,
}

pub enum Json {
    Dict(HashMap<String, Json>),
    List(Vec<Json>),
    Value(Constant),
}

enum DictParseState {
    // represents what was last parsed, so from this, it can be inferred what is
    // expected next. If, for example, the last thing that has been parsed was a key, a value is
    // expected afterwards and vice versa.
    ExpectKey,
    ExpectValue,
    ExpectColon,
    ExpectCommaOrEnd,
}

fn parse_dict(tokens: &[Token]) -> Result<Json, JsonParseError> {
    let mut state = DictParseState::ExpectKey;
    let mut result_hashmap = Hashmap::new();
    let mut prev_key = String::new();
    let mut tokens_iter = tokens.iter();
    while let Some(token) = tokens_iter.next() {
        match state {
            DictParseState::ExpectKey => match token {
                Token::Constant(Constant::StringLiteral(key)) => {
                    prev_key = key.to_string(); // TODO this could potentially be solved more
                                                // efficiently or at least more elegently by pasing ownership of the tokens to the
                                                // function, so into_iter is possible and this copy is not required.
                    state = DictParseState::ExpectColon;
                }
                _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
            },
            DictParseState::ExpectColon => match token {
                Token::Colon => {
                    state = DictParseState::ExpectValue;
                }
                _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
            },
            DictParseState::ExpectValue => match token {
                Token::Constant(c) => {
                    result_hashmap.insert(prev_key, Json::Value(c.clone()));
                    state = DictParseState::ExpectCommaOrEnd;
                }
                Token::StartOfDict => {
                    // find end of dict
                    let mut depth = 1;
                    let mut context_tokens = Vec::new();
                    while let Some(token) = tokens_iter.next() {
                        context_tokens.push(token.clone()); // TODO avoid cloning by taking
                        // ownership of tokens (this function won't need them anymore afterwards)
                    }
                },
                Token::StartOfList => {
                    todo!()
                },
                _ => { return Err(JsonParseError::UnexpectedToken(token.clone())) },
            },
            DictParseState::ExpectCommaOrEnd => match token {
                Token::Comma => {
                    state = DictParseState::ExpectKey;
                }
                Token::EndOfDict => return Ok(Json::Dict(result_hashmap)),
                _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
            },
        }
    }
    Ok(Json::Value(Constant::Null)) // FIXME so the lsp will leave me alone
}

fn parse_list(tokens: &[Token]) -> Json {}

pub fn parse(json: &str) -> Json {}
