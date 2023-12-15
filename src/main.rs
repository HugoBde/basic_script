use std::env::args;
use std::fs;
use std::process::exit;

use basic_script::lexer::Lexer;

fn main() {

    let args: Vec<String> = args().collect();

    if args.len() != 2 {

        println!("Missing input file");

        exit(1);
    }

    let input_file_content = fs::read_to_string(&args[1]).unwrap();

    println!("{}", input_file_content);

    let mut lexer = Lexer::new(input_file_content);

    let lexemes = lexer.lex();

    println!("{:#?}", lexemes);
}
