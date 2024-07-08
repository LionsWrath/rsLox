use clap::Parser;

mod lox;
mod token_type;
mod token;
mod scanner;
mod parser;
mod error;
mod environment;
#[path = "ast/ast_expr.rs"] mod ast_expr;
#[path = "ast/visit_expr.rs"] mod visit_expr;
#[path = "ast/ast_stmt.rs"] mod ast_stmt;
#[path = "ast/visit_stmt.rs"] mod visit_stmt;
#[path = "tools/ast_printer.rs"] mod ast_printer;
#[path = "tools/interpreter.rs"] mod interpreter;


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
