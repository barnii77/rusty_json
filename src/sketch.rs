pub enum DataType {
    Null,
    Float(f64),
    Int(isize),
    StringLit(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseToDtypeError {
    #[error("Cannot parse {0} to DataType")]
    CannotParse(String),
}

impl DataType {
    fn parse(buffer: &str) -> Result<Self, ParseToDtypeError> {
        if buffer.starts_with('"') && buffer.ends_with('"') {
            // string literal
            Ok(Self::StringLit((&buffer[1..buffer.len() - 1]).into()))
        } else if let Ok(integer) = buffer.parse::<isize>() {
            Ok(Self::Int(integer))
        } else if let Ok(float) = buffer.parse::<f64>() {
            Ok(Self::Float(float))
        } else {
            Err(ParseToDtypeError::CannotParse(format!(
                "Cannot parse {} to DataType",
                buffer
            )))
        }
    }
}

pub enum JsonItem {
    NamedNestedValue(String, Vec<JsonItem>),
    NestedValue(Vec<JsonItem>),
    NamedShallowValue(String, DataType),
    ShallowValue(DataType),
}

enum Token {
    StartOfDict,
    StartOfList,
    EndOfDict,
    EndOfList,
    Key(String),
    Value(String),
    Separator,
}

// TODO rewrite parse function so it has 2 stacked while loops iterating over the tokens, the
// inner one starts when it hits a new token, it sets a state (or it is maybe set before the inner
// loop) and keeps going until the token has ended (meaning there has to be a function that returns
// whether a token is finished or not). Once the token is finished, the buffer it was stored into
// is parsed into an actual token, the token is added to the vec of tokens and the next state is
// derived using that last token. This will keep going until all tokens have been captured. 

#[derive(Debug, thiserror::Error)]
pub enum JsonParseError {
    #[error("Unexpected token in following {0}")]
    UnexpectedToken(String),
}

fn parse(json: String) -> Result<JsonItem, JsonParseError> {
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = vec![];
    for char in json.chars() {
        match char {
            '{' => {
                if buffer.is_empty() {
                    tokens.push(Token::StartOfDict);
                } else {
                    let err_msg = format!("Unexpected token {{ in following {}", buffer);
                    return Err(JsonParseError::UnexpectedToken(err_msg));
                }
            }
            '}' => {
                if buffer.is_empty() {
                    tokens.push(Token::EndOfDict);
                } else {
                    let err_msg = format!("Unexpected token }} in following {}", buffer);
                    return Err(JsonParseError::UnexpectedToken(err_msg));
                }
            }
            _ => {
                let err_msg = format!("Unexpected token {} in following {}", char, buffer);
                return Err(JsonParseError::UnexpectedToken(err_msg));
            }
        }
    }
    Ok(JsonItem::ShallowValue(DataType::StringLit(String::from(""))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {}
}
