use crate::datastore::datatypes::DataType;
use crate::datastore::query_proc::QueryResult;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    OpenCurly,
    CloseCurly,
    String(String),
    Number(u64),
    Field(String),
    None,
}

/// Produces a set of Instructions from the raw query string
///
/// # Arguments
///
/// * `query` - the raw query string
///
/// # Example
///
/// ```rust
/// use crate::datastore::query_proc::query_ingestor;
/// let query = String::from("{username:\"johnperry\"}")
/// ingest(&query);
/// ```
pub fn ingest<'a>(query: &String) -> Result<Vec<Instructions>, QueryResult<'a>> {
    match lexer(query) {
        Ok(tokens) => Ok(parser(tokens)),
        Err(e) => Err(e),
    }
}

/// Parses the raw query string and converts it into Tokens
///
/// # Reference
///
/// https://docs.rs/unicode-normalization/0.1.19/unicode_normalization/char/index.html
/// https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html#verify-and-extract-login-from-an-email-address
/// https://adriann.github.io/rust_parser.html
/// https://realpython.com/cpython-source-code-guide/#lexing-and-parsing
/// https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools
///
fn lexer<'a>(query: &String) -> Result<Vec<Token>, QueryResult<'a>> {
    if !query.starts_with('{') || !query.ends_with('}') {
        return Err(QueryResult::InvalidQueryError);
    };
    let mut tokens = Vec::new();
    let mut in_string = false;
    let mut in_field = false;
    let mut in_number = false;
    let mut current_token = String::new();
    for character in query.chars() {
        let token = match character {
            '{' => {
                if in_string {
                    Token::None
                } else {
                    Token::OpenCurly
                }
            }
            '}' => {
                if in_string {
                    current_token.push(character);
                    Token::None
                } else if in_number {
                    in_number = false;
                    let finished_token: u64 = current_token.parse().unwrap();
                    current_token = String::new();
                    tokens.push(Token::Number(finished_token));
                    Token::CloseCurly
                } else {
                    Token::CloseCurly
                }
            }
            '"' => {
                if in_string {
                    in_string = false;
                    let finished_token = current_token.clone();
                    current_token = String::new();
                    Token::String(finished_token)
                } else {
                    in_string = true;
                    Token::None
                }
            }
            ':' => {
                if in_string {
                    current_token.push(character);
                    Token::None
                } else if in_field {
                    in_field = false;
                    let finished_token = current_token.clone();
                    current_token = String::new();
                    Token::Field(finished_token)
                } else {
                    Token::None
                }
            }
            ' ' => {
                if in_string {
                    current_token.push(character);
                    Token::None
                } else if in_field {
                    in_field = false;
                    let finished_token = current_token.clone();
                    current_token = String::new();
                    Token::Field(finished_token)
                } else if in_number {
                    in_number = false;
                    let finished_token: u64 = current_token.parse().unwrap();
                    current_token = String::new();
                    Token::Number(finished_token)
                } else {
                    Token::None
                }
            }
            ',' => {
                if in_string {
                    current_token.push(character);
                    Token::None
                } else if in_number {
                    in_number = false;
                    let finished_token: u64 = current_token.parse().unwrap();
                    current_token = String::new();
                    Token::Number(finished_token)
                } else if in_field {
                    return Err(QueryResult::InvalidQueryError);
                } else {
                    Token::None
                }
            }
            // TODO: Add other number types
            '0' => {
                if in_string || in_field {
                    current_token.push(character);
                    Token::None
                } else {
                    return Err(QueryResult::InvalidQueryError);
                }
            }
            '1'..='9' => {
                if in_string || in_field {
                    current_token.push(character);
                    Token::None
                } else {
                    in_number = true;
                    current_token.push(character);
                    Token::None
                }
            }
            _ => {
                if in_string || in_field {
                    current_token.push(character);
                    Token::None
                } else {
                    in_field = true;
                    current_token.push(character);
                    Token::None
                }
            }
        };
        if token != Token::None {
            tokens.push(token);
        }
    }
    // let filtered = tokens
    //     .into_iter()
    //     .filter(|tok| !matches!(tok, Token::None))
    //     .collect();
    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instructions {
    Equal(String, DataType),
    None,
}

/// Converts a list of Tokens into a set of Instructions to be executed
fn parser(tokens: Vec<Token>) -> Vec<Instructions> {
    let mut previous_token_value = Token::None;
    let mut instructions = Vec::new();
    for token in tokens {
        let instruction = match token {
            Token::Field(_) => {
                previous_token_value = token;
                Instructions::None
            }
            Token::String(val) => {
                if let Token::Field(field) = previous_token_value {
                    previous_token_value = Token::None;
                    Instructions::Equal(field, DataType::String(val))
                } else {
                    Instructions::None
                }
            }
            // TODO: Add other number types
            Token::Number(val) => {
                if let Token::Field(field) = previous_token_value {
                    previous_token_value = Token::None;
                    Instructions::Equal(field, DataType::U64(val))
                } else {
                    Instructions::None
                }
            }
            _ => Instructions::None,
        };
        if instruction != Instructions::None {
            instructions.push(instruction);
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tokens() {
        let query = String::from("{username:\"johnperry\"}");
        let results = lexer(&query).unwrap();
        println!("{:?}", results);
        assert_eq!(
            results,
            vec![
                Token::OpenCurly,
                Token::Field(String::from("username")),
                Token::String(String::from("johnperry")),
                Token::CloseCurly
            ]
        )
    }

    #[test]
    fn get_tokens_2_fields() {
        let query = String::from("{username: \"johnperry\", email:\"johnperry@example.com\"}");
        let results = lexer(&query).unwrap();
        println!("{:?}", results);
        assert_eq!(
            results,
            vec![
                Token::OpenCurly,
                Token::Field(String::from("username")),
                Token::String(String::from("johnperry")),
                Token::Field(String::from("email")),
                Token::String(String::from("johnperry@example.com")),
                Token::CloseCurly
            ]
        )
    }

    #[test]
    fn get_tokens_number() {
        let query = String::from("{username: \"johnperry\", id: 31}");
        let results = lexer(&query).unwrap();
        println!("{:?}", results);
        assert_eq!(
            results,
            vec![
                Token::OpenCurly,
                Token::Field(String::from("username")),
                Token::String(String::from("johnperry")),
                Token::Field(String::from("id")),
                Token::Number(31u64),
                Token::CloseCurly
            ]
        )
    }

    #[test]
    fn get_tokens_2_numbers() {
        let query = String::from("{project_id : 6543,username: \"johnperry\", id: 31}");
        let results = lexer(&query).unwrap();
        println!("{:?}", results);
        assert_eq!(
            results,
            vec![
                Token::OpenCurly,
                Token::Field(String::from("project_id")),
                Token::Number(6543u64),
                Token::Field(String::from("username")),
                Token::String(String::from("johnperry")),
                Token::Field(String::from("id")),
                Token::Number(31u64),
                Token::CloseCurly
            ]
        )
    }

    #[test]
    fn parse_tokens() {
        let query = String::from("{username: \"johnperry\", email:\"johnperry@example.com\"}");
        let results = lexer(&query).unwrap();
        println!("{:?}", results);
        let parsed = parser(results);
        println!("{:?}", parsed);
        assert_eq!(
            parsed,
            vec![
                Instructions::Equal(
                    String::from("username"),
                    DataType::String(String::from("johnperry"))
                ),
                Instructions::Equal(
                    String::from("email"),
                    DataType::String(String::from("johnperry@example.com"))
                )
            ]
        )
    }

    #[test]
    fn process_operations() {
        let query = String::from("{username: \"johnperry\", email:\"johnperry@example.com\"}");
        let results = lexer(&query).unwrap();
        println!("{:?}", results);
        let parsed = parser(results);
        println!("{:?}", parsed);
        assert_eq!(
            parsed,
            vec![
                Instructions::Equal(
                    String::from("username"),
                    DataType::String(String::from("johnperry"))
                ),
                Instructions::Equal(
                    String::from("email"),
                    DataType::String(String::from("johnperry@example.com"))
                )
            ]
        )
    }
}
