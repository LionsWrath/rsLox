use clap::Parser;
use token::Token;
use token_type::TokenType;

mod lox;
mod token_type;
mod token;
mod scanner;
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

    // let expression = Box::new(Expr::BINARY(Binary::new(
    //     Token::new(TokenType::PLUS, 1),
    //     Box::new(
    //         Expr::UNARY(Unary::new(
    //             Token::new(TokenType::MINUS, 1),
    //             Box::new(Expr::LITERAL(Token::new(TokenType::NUMBER(45.0), 1)))
    //         )
    //     )),
    //     Box::new(
    //         Expr::GROUPING(
    //             Grouping::new(
    //                 Box::new(
    //                     Expr::LITERAL(Token::new(TokenType::NUMBER(85.5), 1))
    //                 )
    //             )
    //         )
    //     )
    // )));

    // let mut ast_printer = ast_printer::AstPrinter::new();
    // println!("{}", ast_printer.printer(&expression));
    
}
