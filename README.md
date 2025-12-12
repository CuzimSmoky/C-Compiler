# C Compiler in Rust

This project is a simple **C compiler written in Rust**, currently supporting basic lexing for a subset of C code.

## Project Structure

- **`compiler`** – The main Rust project folder
- **`driver.rs`** – Entry point that runs the compiler
- **`lexer.rs`** – Performs lexical analysis and generates tokens

## Current Features

- Lexical analysis of simple C programs
- Supports the following tokens:
  - Keywords: `int`, `void`, `return`
  - Identifiers (e.g., `main`)
  - Constants (integer numbers)
  - Symbols: `(`, `)`, `{`, `}`, `;`
- Prints all tokens in order for debugging
- 
## To be added:

  - Parser
  - Code Emission
  - more Syntax
