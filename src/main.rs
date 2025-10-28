mod lexer;
mod parser;

use clap::Parser as ClapParser; // Rename to avoid conflict with your Parser struct
use lexer::Lexer;
use parser::{Parser, Type};
use std::fs;

#[derive(ClapParser)]
struct Args {
    input: String,
    #[arg(short, long)]
    file: bool,
}
fn main() {
    let args = Args::parse();
    let input = if args.file {
    match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Could not read file '{}': {}", args.input, err);
            std::process::exit(1);
        }
    }
} else {
    args.input.clone()
};


    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();
    println!("Tokens: {tokens:?}");

    let mut parser = Parser::new(tokens.into_iter());
   match parser.parse() {
    Ok(ast) => {
        println!("\n Parsed structure:");
        pretty_print(&ast, 0);
    }
    Err(e) => {
        eprintln!("Parse error: {e}");
    }
}

}

fn pretty_print(value: &Type, indent: usize) {
    let space = "  ".repeat(indent);
    match value {
        Type::Number(n) => println!("{space}{n}"),
        Type::String(s) => println!("{space}\"{s}\""),
        Type::Boolean(b) => println!("{space}{b}"),
        Type::Null => println!("{space}null"),
        Type::Array(vals) => {
            println!("{space}[");
            for v in vals {
                pretty_print(v, indent + 1);
            }
            println!("{space}]");
        }
        Type::Object(pairs) => {
            println!("{space}{{");
            for (k, v) in pairs {
                print!("{space}  \"{k}\": ");
                match v {
                    Type::Object(_) | Type::Array(_) => {
                        println!();
                        pretty_print(v, indent + 2);
                    }
                    _ => pretty_print(v, 0),
                }
            }
            println!("{space}}}");
        }
    }
}

