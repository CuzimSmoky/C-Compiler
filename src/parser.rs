use std::{env::consts::EXE_SUFFIX, process::exit};

use crate::{ lexer, token::{self, Token}};

pub fn run(tokens: &mut Vec<Token>) {
    println!("Parser is running");
    let program  = parse_program(tokens);
    println!("AST: {:?}", program);
}

pub fn parse_program(tokens: &mut Vec<Token>) -> Program{
    let function = parse_function(tokens);
    let program = Program {
        function: function,
    };
    return program;
}

pub fn parse_function(tokens: &mut Vec<Token>) -> Function {
    let token_type = expect(&Token::KeywordInt, tokens);
    let token_identifier = expect(&Token::Identifier(String::new()), tokens);
    let token_lparen = expect(&Token::LParen, tokens);
    let token_void = expect(&Token::KeywordVoid, tokens);
    let token_rparen = expect(&Token::RParen, tokens);
    let token_lbrace = expect(&Token::LBrace, tokens);
    let statement = parse_statement(tokens);
    let token_rbrace = expect(&Token::RBrace, tokens);
    
    if let Token::Identifier(name) = token_identifier {
        let identifier = Identifier {
            name: name,
        };
        let function = Function {
            name: identifier,
            body: statement,
        };
        return function;
    } else {
        panic!("Expected a Identifier token, found {:?}", token_identifier);
    }

}

pub fn parse_statement(tokens:&mut Vec<Token>) -> Statement {
    let token = expect(&Token::KeywordReturn, tokens);
    let return_val = parse_expression(tokens);
    let token_semicolon = expect(&Token::Semicolon, tokens);
    let expression = Expression::Constant(return_val);
    return Statement::Return(expression);
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> i64 {
    let token = expect(&Token::Constant(0), tokens);
    if let Token::Constant(value) = token {
        return value;
    } else {
        panic!("Expected a Constant token, found {:?}", token);
    }

}

pub fn expect(expected_token: &Token, tokens: &mut Vec<Token>) -> Token {
    let actual_token = tokens.remove(0); // Token konsumieren

    match expected_token {
        Token::Identifier(_) => {
            if !matches!(actual_token, Token::Identifier(_)) {
                panic!("Syntax Error: Expected Identifier, found {:?}", actual_token);
            }
        }
        Token::Constant(_) => {
            if !matches!(actual_token, Token::Constant(_)) {
                panic!("Syntax Error: Expected Constant, found {:?}", actual_token);
            }
        }
        _ => {
            if &actual_token != expected_token {
                panic!("Syntax Error: Expected {:?}, found {:?}", expected_token, actual_token);
            }
        }
    }

    actual_token
}

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}
#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub body: Statement,
}
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}
#[derive(Debug)]
pub enum Expression {
    Constant(i64),
}
#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}