use std::fs::File;
use std::io;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(String),
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

pub struct Tokenizer {
    patterns: Vec<(Regex, Token)>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let patterns = vec![
            (Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(), Token::Identifier(String::new())),
            (Regex::new(r"^[0-9]+\b").unwrap(), Token::Constant(String::new())),
            (Regex::new(r"^int\b").unwrap(), Token::IntKeyword),
            (Regex::new(r"^void\b").unwrap(), Token::VoidKeyword),
            (Regex::new(r"^return\b").unwrap(), Token::ReturnKeyword),
            (Regex::new(r"^\(").unwrap(), Token::OpenParen),
            (Regex::new(r"^\)").unwrap(), Token::CloseParen),
            (Regex::new(r"^\{").unwrap(), Token::OpenBrace),
            (Regex::new(r"^\}").unwrap(), Token::CloseBrace),
            (Regex::new(r"^;").unwrap(), Token::Semicolon),
        ];
        Tokenizer { patterns }
    }

    pub fn tokenize(&self, input: &str) -> io::Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut remaining = input.trim();

        while !remaining.is_empty() {
            let mut matched = false;

            for (pattern, token_type) in &self.patterns {
                if let Some(mat) = pattern.find(remaining) {
                    let token = match token_type {
                        Token::Identifier(_) => Token::Identifier(mat.as_str().to_string()),
                        Token::Constant(_) => Token::Constant(mat.as_str().to_string()),
                        _ => token_type.clone(),
                    };
                    tokens.push(token);
                    remaining = &remaining[mat.end()..].trim();
                    matched = true;
                    break;
                }
            }

            if !matched {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, 
                    format!("Unexpected character sequence: {}", remaining)));
            }
        }

        Ok(tokens)
    }
}

pub fn read_source_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;
    Ok(contents)
}
