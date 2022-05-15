#![allow(dead_code)]
use std::env;
use std::fs::File;
use std::io::{BufWriter, stdout, Write};
use lexer::token::TokenClass;
use crate::ast::ast_node::ASTNode;
use crate::ast::ast_printer::ASTPrinter;

use crate::lexer::scanner::Scanner;
use crate::lexer::tokenizer::Tokenizer;
use crate::parser::parser::Parser;

mod lexer;
mod parser;
mod ast;

static FILE_NOT_FOUND: i32 = 2;
static MODE_FAIL: i32      = 254;
static LEXER_FAIL: i32     = 250;
static PARSER_FAIL: i32    = 245;
static SEM_FAIL: i32       = 240;
static PASS: i32           = 0;

enum Mode {
    LEXER, PARSER, AST, SEMANTICANALYSIS, GEN
}

fn usage() -> ! {
    println!("Usage: rustc main.rs pass inputfile (outputfile)");
    println!("where pass is either: -lexer, -parser, -ast, -sem or -gen");
    std::process::exit(-1);
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 && args.len() != 3 {
        usage();
    }

    let mode;
    let flag = &args[1];
    match &flag[..] {
        "-lexer"  => mode = Mode::LEXER,
        "-parser" => mode = Mode::PARSER,
        "-ast"    => mode = Mode::AST,
        "-sem"    => mode = Mode::SEMANTICANALYSIS,
        "-gen"    => mode = Mode::GEN,
        _          => usage()
    }

    let input_file;
    match File::open(&args[2]) {
        Ok(f) => input_file = f,
        Err(_) => {
            println!("File {:?} does not exist.", args[2]);
            std::process::exit(FILE_NOT_FOUND);
        },
    }

    let scanner = Scanner::new(input_file);
    let mut tokenizer = Tokenizer::new(scanner);

    match mode {
        Mode::LEXER =>  {
            let mut t = tokenizer.next_token();
            while !matches!(t.token_class, TokenClass::EOF) {
                println!("{}", t);
                t = tokenizer.next_token();
            }

            if tokenizer.get_error_count() == 0 {
                println!("Lexing: pass");
                std::process::exit(PASS);
            } else {
                println!("Lexing: failed ({} errors)", tokenizer.get_error_count());
                std::process::exit(LEXER_FAIL);
            }
        },
        Mode::PARSER =>  {
            let mut parser = Parser::new(tokenizer);
            parser.parse();
            if parser.get_error_count() == 0 {
                println!("Parsing: pass");
                std::process::exit(PASS);
            } else {
                println!("Parsing: failed ({} errors)", parser.get_error_count());
                std::process::exit(PARSER_FAIL);
            }
        },
        Mode::AST => {
            let mut stream = BufWriter::new(Box::new(stdout()));

            for i in 0..10 {
                stream.write(&[i+1]).unwrap();
            }
            stream.flush().unwrap();


            let mut parser = Parser::new(tokenizer);
            let mut program_ast = parser.parse();

            if  parser.get_error_count() == 0 {
                // let writer = BufWriter::new(File::create(&path).unwrap());
                let writer = BufWriter::new(stdout());

                program_ast.accept(&mut ASTPrinter::new(writer));

                std::process::exit(PASS)
            } else {
                println!("Parsing: failed ({} errors)", parser.get_error_count());
                std::process::exit(PARSER_FAIL)
            }
        },
        Mode::SEMANTICANALYSIS => {
            println!("Semantic analysis not implemented");
            std::process::exit(MODE_FAIL)
        },
        Mode::GEN => { 
            println!("Code generation not implemented");
            std::process::exit(MODE_FAIL)
        },
    }
}
