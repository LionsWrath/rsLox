use clap::Parser;

mod lox;
mod token_type;
mod token;
mod scanner;
mod parser;
mod error;
#[path = "tools/ast.rs"] mod ast;
#[path = "tools/visit.rs"] mod visit;
#[path = "tools/ast_printer.rs"] mod ast_printer;

use ast::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    input_filename: Option<std::path::PathBuf>,
}


fn main () {

    let args = Args::parse();
    let mut lox = lox::Lox::new();

    match args.input_filename {
        Some(filename) => lox.run_file(&filename),
        None => lox.run_prompt(),
    }  
    
}
