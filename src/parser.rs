use crate::lexer::{tokenize, Constant, JsonLexError, Token};
use std::collections::HashMap;
use thiserror::Error;

// TODO create a datastructure that can store the json
// TODO create a parser that can parse the tokenized json to this datastructure

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("JsonLexError: {0}")]
    JsonLexError(JsonLexError),
    #[error("JsonParseError: {0}")]
    JsonParseError(JsonParseError),
}

#[derive(Debug, Error)]
pub enum JsonParseError {
    #[error("JsonParseError: Invalid token: {0}")]
    UnexpectedToken(Token),
    #[error("JsonParseError: Unexpected end of input")]
    UnexpectedEndOfInput,
}

#[derive(Debug, PartialEq)]
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

enum ListParseState {
    ExpectValue,
    ExpectCommaOrEnd,
}

fn parse_dict(tokens: &[Token]) -> Result<Json, JsonParseError> {
    let mut state = DictParseState::ExpectKey;
    let mut result_hashmap = HashMap::new();
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
            DictParseState::ExpectValue => {
                match token {
                    Token::Constant(c) => {
                        result_hashmap.insert(prev_key.clone(), Json::Value(c.clone()));
                    }
                    Token::StartOfDict => {
                        // find end of dict
                        let mut depth = 1;
                        let mut context_tokens = Vec::new();
                        while let Some(token) = tokens_iter.next() {
                            match token {
                                Token::StartOfDict => depth += 1,
                                Token::EndOfDict => depth -= 1,
                                _ => {}
                            }
                            if depth <= 0 {
                                break;
                            }
                            context_tokens.push(token.clone()); // TODO avoid cloning by taking
                                                                // ownership of tokens (this function won't need them anymore afterwards)
                        }
                        let sub_json = parse_dict(&context_tokens)?;
                        result_hashmap.insert(prev_key.clone(), sub_json);
                    }
                    Token::StartOfList => {
                        // find end of dict
                        let mut depth = 1;
                        let mut context_tokens = Vec::new();
                        while let Some(token) = tokens_iter.next() {
                            match token {
                                Token::StartOfList => depth += 1,
                                Token::EndOfList => depth -= 1,
                                _ => {}
                            }
                            if depth <= 0 {
                                break;
                            }
                            context_tokens.push(token.clone()); // TODO avoid cloning by taking
                                                                // ownership of tokens (this function won't need them anymore afterwards)
                        }
                        let sub_json = parse_list(&context_tokens)?;
                        result_hashmap.insert(prev_key.clone(), sub_json);
                    }
                    _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
                }
                state = DictParseState::ExpectCommaOrEnd;
            }
            DictParseState::ExpectCommaOrEnd => match token {
                Token::Comma => {
                    state = DictParseState::ExpectKey;
                }
                Token::EndOfDict => return Ok(Json::Dict(result_hashmap)),
                _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
            },
        }
    }
    Ok(Json::Dict(result_hashmap))
}

fn parse_list(tokens: &[Token]) -> Result<Json, JsonParseError> {
    let mut state = ListParseState::ExpectValue;
    let mut result_vec: Vec<Json> = Vec::new();
    let mut tokens_iter = tokens.iter();
    while let Some(token) = tokens_iter.next() {
        match state {
            ListParseState::ExpectValue => {
                match token {
                    Token::Constant(c) => result_vec.push(Json::Value(c.clone())),
                    Token::StartOfDict => {
                        // find end of dict
                        let mut depth = 1;
                        let mut context_tokens = Vec::new();
                        while let Some(token) = tokens_iter.next() {
                            match token {
                                Token::StartOfDict => depth += 1,
                                Token::EndOfDict => depth -= 1,
                                _ => {}
                            }
                            if depth <= 0 {
                                break;
                            }
                            context_tokens.push(token.clone()); // TODO avoid cloning by taking
                                                                // ownership of tokens (this function won't need them anymore afterwards)
                        }
                        let sub_json = parse_dict(&context_tokens)?;
                        result_vec.push(sub_json);
                    }
                    Token::StartOfList => {
                        // find end of dict
                        let mut depth = 1;
                        let mut context_tokens = Vec::new();
                        while let Some(token) = tokens_iter.next() {
                            match token {
                                Token::StartOfList => depth += 1,
                                Token::EndOfList => depth -= 1,
                                _ => {}
                            }
                            if depth <= 0 {
                                break;
                            }
                            context_tokens.push(token.clone()); // TODO avoid cloning by taking
                                                                // ownership of tokens (this function won't need them anymore afterwards)
                        }
                        let sub_json = parse_list(&context_tokens)?;
                        result_vec.push(sub_json);
                    }
                    _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
                }
                state = ListParseState::ExpectCommaOrEnd;
            }
            ListParseState::ExpectCommaOrEnd => match token {
                Token::Comma => state = ListParseState::ExpectValue,
                _ => return Err(JsonParseError::UnexpectedToken(token.clone())),
            },
        }
    }
    Ok(Json::List(result_vec))
}

pub fn parse(json: &str) -> Result<Json, JsonError> {
    let tokens_result = tokenize(json);
    let tokens: Vec<Token> = match tokens_result {
        Ok(tokens) => tokens,
        Err(e) => return Err(JsonError::JsonLexError(e)),
    };
    if tokens.len() < 2 {
        // If there is only one token, there cannot even be a pair of opening and
        // closing brackets. Therefor, must be invalid. This check is mostly there for avoiding
        // future leg-shooting if I change something
        return Err(JsonError::JsonParseError(
            JsonParseError::UnexpectedEndOfInput,
        ));
    }
    let first_token = &tokens[0]; // safe because length is checked
    let last_token = &tokens[tokens.len() - 1]; // safe because length is checked
    let remaining_tokens = &tokens[1..tokens.len() - 1];

    match first_token {
        Token::StartOfDict => {
            if last_token != &Token::EndOfDict {
                return Err(JsonError::JsonParseError(JsonParseError::UnexpectedToken(
                    last_token.clone(),
                )));
            }
            let result = parse_dict(remaining_tokens);
            match result {
                Ok(r) => Ok(r),
                Err(e) => Err(JsonError::JsonParseError(e)),
            }
        }
        Token::StartOfList => {
            if last_token != &Token::EndOfList {
                return Err(JsonError::JsonParseError(JsonParseError::UnexpectedToken(
                    last_token.clone(),
                )));
            }
            let result = parse_list(remaining_tokens);
            match result {
                Ok(r) => Ok(r),
                Err(e) => Err(JsonError::JsonParseError(e)),
            }
        }
        _ => Err(JsonError::JsonParseError(JsonParseError::UnexpectedToken(
            first_token.clone(),
        ))),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_valid_json() {
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
        let parsed = match parse(json) {
            Ok(p) => p,
            Err(e) => panic!("{:?}", e),
        };

        let expected_json = {
            let mut expected_output = HashMap::new();
            expected_output.insert("key1".to_string(), Json::Value(Constant::Int(1)));
            expected_output.insert(
                "key2".to_string(),
                Json::Value(Constant::StringLiteral("value2".to_string())),
            );
            expected_output.insert(
                "key3".to_string(),
                Json::List(vec![
                    Json::Value(Constant::Int(1)),
                    Json::Value(Constant::Int(2)),
                    Json::Value(Constant::Int(3)),
                ]),
            );
            let mut key4_map = HashMap::new();
            key4_map.insert(
                "key5".to_string(),
                Json::Value(Constant::StringLiteral("value5".to_string())),
            );
            key4_map.insert("key6".to_string(), Json::Value(Constant::Int(6)));
            expected_output.insert("key4".to_string(), Json::Dict(key4_map));
            Json::Dict(expected_output)
        };
        assert_eq!(parsed, expected_json);
        println!("{:?}", parsed);
    }

    #[test]
    #[should_panic]
    fn test_full_invalid_json() {
        // FIXME figure out why it panics is the json is invalid
        let json = r#"
        {
            "key1": 1,
            "key2": "value2",
            "key3": [1, 2, 3],
            "key4": {
                "key5": "value5",
                key6": 6
            }
        }
        "#;
        let parsed = match parse(json) {
            Ok(p) => p,
            Err(e) => panic!("{:?}", e),
        };

        let expected_json = {
            let mut expected_output = HashMap::new();
            expected_output.insert("key1".to_string(), Json::Value(Constant::Int(1)));
            expected_output.insert(
                "key2".to_string(),
                Json::Value(Constant::StringLiteral("value2".to_string())),
            );
            expected_output.insert(
                "key3".to_string(),
                Json::List(vec![
                    Json::Value(Constant::Int(1)),
                    Json::Value(Constant::Int(2)),
                    Json::Value(Constant::Int(3)),
                ]),
            );
            let mut key4_map = HashMap::new();
            key4_map.insert(
                "key5".to_string(),
                Json::Value(Constant::StringLiteral("value5".to_string())),
            );
            key4_map.insert("key6".to_string(), Json::Value(Constant::Int(6)));
            expected_output.insert("key4".to_string(), Json::Dict(key4_map));
            Json::Dict(expected_output)
        };
        assert_eq!(parsed, expected_json);
        println!("{:?}", parsed);
    }
}
