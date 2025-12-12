use std::fs;
use std::path::Path;
use regex::Regex;

pub fn run(file_path: &Path) {
    println!("Lexer is running");
    let mut token_vec: Vec<Token>  = Vec::new();

    let file_content = fs::read_to_string(file_path)
        .expect("Lex-Error while opening File");
    let mut file_content_string: &str = file_content.as_str();

    while !file_content_string.is_empty() {
        let trimmed_string = file_content_string.trim_start();
        file_content_string = &file_content_string[file_content_string.len()-trimmed_string.len()..];
        if !file_content_string.is_empty() {
            let (identifier, len) = search_identifier(file_content_string);
            token_vec.push(identifier);
            file_content_string = &file_content_string[len..];
            println!("Tokens: {:?}", token_vec);
        } else {
            println!("Finished leing the input successfully");
        }

    }
}

pub fn search_identifier(input: &str) -> (Token, usize){
    let first_char = &input.chars().next().unwrap();
    match first_char {
        '(' => return (Token::LParen, 1),
        ')' => return (Token::RParen, 1),
        '{' => return (Token::LBrace, 1),
        '}' => return (Token::RBrace, 1),
        ';' => return (Token::Semicolon, 1),
        _ => {
                let regex_identifier = Regex::new(r"^[a-zA-Z_]\w*\b").unwrap();
                let regex_constant:Regex = Regex::new(r"^[0-9]+\b").unwrap();
                let is_identifier: bool = regex_identifier.is_match(&input);
                let found_match;
                let token: &str;
                let matched_token: Token;

                if is_identifier {
                    found_match = regex_identifier.find(&input).expect("No match found for identiefier");
                    token = &input[found_match.start()..found_match.end()];
                    matched_token = match_identifier(&token);
                } else {
                    found_match = regex_constant.find(&input).expect("No match found for constant");
                    token = &input[found_match.start()..found_match.end()];
                    matched_token = match_constant(&token);
                }
                return (matched_token, token.len());
        }
    }
}
pub fn match_identifier(identifier: &str) -> Token {
    match identifier {
        "int" => return Token::KeywordInt,
        "void" => return Token::KeywordVoid,
        "return" => return Token::KeywordReturn,
        _ => {
            let identifier_string = identifier.to_string();
            return Token::Identifier(identifier_string);
        }
    }
}
pub fn match_constant(identifier: &str) -> Token {
    let identifier_as_int = identifier.parse::<i64>().unwrap();
    return Token::Constant(identifier_as_int);
}
#[derive(Debug)]
pub enum Token {
    KeywordInt,
    KeywordVoid,
    KeywordReturn,
    Identifier(String),
    Constant(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
}
