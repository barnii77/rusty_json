#[test]
fn parse_json() {
    let raw_json = r#"
        [{"id":"2489651045","type":"CreateEvent","actor":{"id":665991,"login":"petroav","gravatar_id":"","url":"https://api.github.com/users/petroav","avatar_url":"https://avatars.githubusercontent.com/u/665991?"}]
    "#;
    let json = crate::parser::parse(raw_json).expect("should not error");
    match json {
        crate::parser::Json::List(x) => {
            match &x[0] {
                crate::parser::Json::Dict(x) => {
                    match x.get("actor").unwrap() {
                        crate::parser::Json::Dict(x) => {
                            match x.get("login").unwrap() {
                                crate::parser::Json::Value(crate::lexer::Constant::StringLiteral(x)) => {
                                    assert_eq!(x, "petroav");
                                    println!("login: {}", x);
                                }
                                _ => panic!("should be a string"),
                            }
                        }
                        _ => panic!("should be a dict"),
                    }
                }
                _ => panic!("should be a dict"),
            }
        }
        _ => panic!("should be a list"),
    }
}
