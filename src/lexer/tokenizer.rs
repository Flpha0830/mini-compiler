use std::{collections::HashMap, io::ErrorKind};
use crate::{lexer::token::{Token, TokenClass}, Scanner};

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err(e),
        }
    }
}

pub struct Tokenizer {
    scanner: Scanner,
    error: i32,
    char_map: HashMap<char, TokenClass>,
    key_map: HashMap<&'static str, TokenClass>,
    escape_map: HashMap<&'static str, &'static str>,
}

impl Tokenizer {
    pub fn new(scanner: Scanner) -> Self {
        Tokenizer {
            scanner,
            error: 0,
            char_map: HashMap::from([
                ('{', TokenClass::LBRA), ('}', TokenClass::RBRA), ('(', TokenClass::LPAR), (')', TokenClass::RPAR),
                ('[', TokenClass::LSBR), (']', TokenClass::RSBR), (';', TokenClass::SC), (',', TokenClass::COMMA),
                ('+', TokenClass::PLUS), ('-', TokenClass::MINUS), ('*', TokenClass::ASTERIX), ('/', TokenClass::DIV),
                ('%', TokenClass::REM), ('.', TokenClass::DOT)
            ]),
            key_map: HashMap::from([
                ("int", TokenClass::INT), ("void", TokenClass::VOID), ("char", TokenClass::CHAR),
                // keywords
                ("if",TokenClass::IF), ("else",TokenClass::ELSE), ("while",TokenClass::WHILE),
                ("return",TokenClass::RETURN), ("struct",TokenClass::STRUCT), ("sizeof",TokenClass::SIZEOF)
            ]),
            escape_map: HashMap::from([
                ("\\t", "\t"),
                ("\\b", r"\b"),
                ("\\n", "\n"),
                ("\\r", "\r"),
                ("\\f", r"\f"),
                ("\\'", "'"),
                ("\\\"" , "\""),
                ("\\\\", "\\"),
                ("\\0", "\0")
            ])
        }
    }

    pub fn get_error_count(&self) -> i32 {
        self.error
    }

    fn error(&mut self, c: char, line: i32, col: i32) {
        println!("Lexing error: unrecognised character ({}) at {}: {}", c, line, col);
        self.error += 1;
    }

    pub fn next_token(&mut self) -> Token {
        match self.next() {
            Ok(t) => return t,
            Err(_e@ErrorKind::UnexpectedEof) => 
                return Token::new(TokenClass::EOF, "", self.scanner.get_line(), self.scanner.get_column()),
            Err(_) => std::process::exit(-1),   
        };
    }

    fn next(&mut self) -> Result<Token, ErrorKind> {

        let line = self.scanner.get_line();
        let column = self.scanner.get_column();
        let scanner = &mut self.scanner;


        
        // get the next character
        let mut c = unwrap_or_return!(scanner.next());

        // skip white spaces
        if c == ' ' {
            return self.next()
        }

        if c == '/' {
            let t = unwrap_or_return!(scanner.peek());
            if t == '/' {
                while scanner.get_line() == line { 
                    unwrap_or_return!(scanner.next());
                };
                return self.next();
            } else if t == '*' {
                unwrap_or_return!(scanner.next());
                loop {
                    c = unwrap_or_return!(scanner.next());
                    if c == '*' && unwrap_or_return!(scanner.peek()) == '/' {
                        break;
                    }
                };
                unwrap_or_return!(scanner.next());
                return self.next();
            }
        }



        // if we reach this point, it means we did not recognise a valid token
        self.error(c, line, column);
        Ok(Token::new(TokenClass::INVALID, "", line, column))
    }
}
















