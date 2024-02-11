use std::io::Write;

use crate::{
    lexer::Lexer,
    token::{
        TokenKind,
        Symbol,
    },
};

pub mod lexer;
pub mod token;
pub mod error;

// Test only
fn main(
    // no args
) {
    let mut cd = std::env::current_dir()
        .expect("Failed to retrieve current directory");

    cd.extend(["src"]);
    println!("Current directory: {cd:?}");
    
    for file in cd.read_dir()
        .expect("Failed to read source directory")
        .flatten()
    {
        let file_path = file.path();
        match file_path.extension().and_then(|s| s.to_str()) {
            Some("rs") => {},
            None | Some(_) => {
                println!("Skipping non source-file {file_path:?}");
                continue;
            }
        }

        let mut out_path = file_path.clone();
        out_path.set_extension("tokens");

        let mut out_file = match std::fs::OpenOptions::new()
            .create(true).create_new(false)
            .write(true).truncate(true)
            .read(false)
            .open(&out_path)
        {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to open output file {out_path:?}");
                println!("{err:?}");
                continue;
            },
        };

        let file_content = match std::fs::read(&file_path) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(text) => text,
                Err(err) => {
                    println!("Contents of file {file_path:?} is not valid UTF-8)");
                    println!("{err:?}");
                    continue;
                },
            },
            Err(err) => {
                println!("Failed to read source file {file_path:?}");
                println!("{err:?}");
                continue;
            },
        };

        println!("Parsing file: {file_path:?}");
        let mut lexer = Lexer::new(&file_content);

        println!("Token stream to follow");
        println!("======================");

        while let Ok(token) = lexer.next_token()
        {
            if let Err(err) = match token.kind()
            {
                TokenKind::Keyword(kw) => out_file.write_fmt(format_args!("KeyWord\t{kw}\n")),
                TokenKind::Symbol(Symbol::EndOfInput) => break,
                TokenKind::Symbol(sym) => out_file.write_fmt(format_args!("Symbol\t{sym}\n")),
                TokenKind::Identifier(ident) => out_file.write_fmt(format_args!("Identifier\t{ident}\n")),
                TokenKind::Number(num) => out_file.write_fmt(format_args!("Number\t{num}\n")),
            } {
                println!("Error writing token to file, {err:?}");
                break;
            }
        }

        if let Err(err) = out_file.flush() {
            println!("Failed to flush file {out_path:?}");
            println!("{err:?}");
        }
    }
}
