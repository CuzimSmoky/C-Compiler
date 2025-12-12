use std::env;
use std::path::Path;
use crate::parser;
use crate::lexer;

pub fn run() {
    // Creating a vector containing all arguments passed to the compiler
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        eprint!("Error: not enough arguments. Please use cargo run ./mycc <path/to/file>");
    }
    // Extracting the path to file from the env args
    let path_to_file_as_string: &str = &args[1];
    let path_to_file = Path::new(path_to_file_as_string);
    let preprocessed_file = path_to_file.with_extension("i");

    // If additional args got passed match them to their corresponding usage
    if args.len() == 3 {
        let arg: &str = &args[2];
            match arg {
                "--lex" => {
                    lexer::run(&path_to_file);
                    std::process::exit(0);
                }
                "--parse" => {
                    lexer::run(&path_to_file);
                    parser::run();
                    std::process::exit(0);
                }
                "--codegen" => {
                    lexer::run(&path_to_file); 
                    parser::run();
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("Unknown argument");
                    std::process::exit(1);
                }
            }
    }
    // creating a .i file from the original .c file
    let preprocessing_status = std::process::Command::new("gcc")
        .args([
            "-E",
            "-P",
            path_to_file_as_string,
            "-o",
            preprocessed_file.to_str().unwrap()
        ])
        .status()
        .expect("Failed to start gcc");

    if !preprocessing_status.success() {
        eprint!("Error during preprocessing");
        std::process::exit(1)
    }

    
    let assembled_file = path_to_file.with_extension("s");
    let assembled_file_path_as_string = assembled_file.to_str().unwrap();
    let output_file = path_to_file.with_extension("");
    let output_file_as_string = output_file.to_str().unwrap();

    // Turning te .i file into  assembly
    let assembling_status = std::process::Command::new("gcc")
        .args([
            assembled_file_path_as_string,
            "-o",
            output_file_as_string
        ])
        .status()
        .expect("Failed to start gcc");

    if !assembling_status.success() {
        eprint!("Error during assembling");
        std::process::exit(1)
    }

    if preprocessing_status.success() && assembling_status.success() {
        std::fs::remove_file(preprocessed_file).ok();
        std::fs::remove_file(assembled_file).ok();
    }
}