use exitcode;
use std::io;
use std::path::PathBuf;
use std::process;

use crate::ast::Expr;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::ast_printer::AstPrinter;

#[path = "utils/utils.rs"] mod utils;

pub struct Lox {
    has_error: bool,
}

impl Lox {

    pub fn new() -> Self {
        Lox {
            has_error: false
        }
    }

    pub fn run_file(&self, filename: &PathBuf) {
        self.run(utils::read_file(filename));

        if self.has_error {
            process::exit(exitcode::DATAERR);
        }
    }

    pub fn run_prompt(&mut self) {

        let stdin = io::stdin();
        let mut line = String::new();

        loop {
            line.clear();
            self.has_error = false;
            match stdin.read_line(&mut line) {
                Ok(_) => {
                    let trimmed = line.trim_end();
                    self.run(trimmed.chars().collect::<Vec<_>>());
                },
                Err(_) => break,
            }
        }

    }

    fn run(&self, source: Vec<char>) {
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new(scanner.scan_tokens().clone());

        let expr: Expr = parser.parse();

        let mut ast_printer = AstPrinter::new();
        println!("{}", ast_printer.printer(&expr));
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.log(line, String::new(), message);
        self.has_error = true;
    }

    pub fn log(&self, line: usize, doko: String, message: String) {
        println!("[line {line}] Error {doko}: {message}");
    }

}
