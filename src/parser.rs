use crate::lexer::{JsonLexError, Token, Tokenizer, Constant};
use std::collections::HashMap;

// TODO create a datastructure that can store the json
// TODO create a parser that can parse the tokenized json to this datastructure 
 
pub enum Json {
    Dict(HashMap<String, Json>),
    List(Vec<Json>),
    Value(Constant),
}

enum ParseContext {
    Dict,
    List,
}
