mod lexer;

use clap::Parser;
use std::path::PathBuf;
use lexer::{Tokenizer, read_source_file};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "FILE")]
    input: PathBuf,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,
}

fn main() {
    let args = Args::parse();

    // Print the input file path
    println!("Input file: {}", args.input.display());

    // Check and print each flag
    if args.lex {
        println!("Performing lexical analysis...");
        let tokenizer = Tokenizer::new();
        match read_source_file(args.input.to_str().unwrap()) {
            Ok(input) => {
                match tokenizer.tokenize(&input) {
                    Ok(tokens) => println!("Tokens: {:#?}", tokens),
                    Err(e) => eprintln!("Lexical error: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }

    if args.parse {
        println!("Parsing enabled");
    }

    if args.codegen {
        println!("Code generation enabled");
    }
}