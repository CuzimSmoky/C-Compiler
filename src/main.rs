mod driver;
mod lexer;
mod parser;
mod token;
mod ast;
mod asm_ast;
mod codegen;

fn main() {
    driver::run();
}
