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
        if c.is_whitespace() {
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

        if self.char_map.contains_key(&c) {
            return Ok(Token::new(self.char_map.get(&c).unwrap().clone(), "", line, column));
        }

        match (c, unwrap_or_return!(scanner.peek())) {
            ('&','&') => { unwrap_or_return!(scanner.next()); return Ok(Token::new(TokenClass::LOGAND, "", line, column)) },
            ('=','=') => { unwrap_or_return!(scanner.next()); return Ok(Token::new(TokenClass::EQ, "", line, column)) },
            ('|','|') => { unwrap_or_return!(scanner.next()); return Ok(Token::new(TokenClass::LOGOR, "", line, column)) },
            ('!','=') => { unwrap_or_return!(scanner.next()); return Ok(Token::new(TokenClass::NE, "", line, column)) },
            ('<','=') => { unwrap_or_return!(scanner.next()); return Ok(Token::new(TokenClass::LE, "", line, column)) },
            ('>','=') => { unwrap_or_return!(scanner.next()); return Ok(Token::new(TokenClass::GE, "", line, column)) },
            ('&', _ ) => return Ok(Token::new(TokenClass::AND, "", line, column)),
            ('=', _ ) => return Ok(Token::new(TokenClass::ASSIGN, "", line, column)),
            ('<', _ ) => return Ok(Token::new(TokenClass::LT, "", line, column)),
            ('>', _ ) => return Ok(Token::new(TokenClass::GT, "", line, column)),
            _ => { }
        }

        if c.is_ascii_digit() {
            let mut data = String::new();
            data.push(c);
            c = unwrap_or_return!(scanner.peek());
            while c.is_ascii_digit() {
                data.push(c);
                unwrap_or_return!(scanner.next());
                c = unwrap_or_return!(scanner.peek());
            }

            return Ok(Token::new(TokenClass::INTLITERAL, data.as_str(), line, column));
        }

        if c == '\'' {
            if(unwrap_or_return!(scanner.peek()) == '\'') {
                self.error(c, line, column);
                return Ok(Token::new(TokenClass::INVALID, "",line, column));
            }

            let mut data = String::new();
            c = unwrap_or_return!(scanner.next());
            while c != '\'' || data.eq("\\") {
                data.push(c);
                c = unwrap_or_return!(scanner.next());
            }

            if data.len() == 1 {
                return Ok(Token::new(TokenClass::CHARLITERAL, data.as_str(), line, column));
            }

            if self.escape_map.contains_key(data.as_str()) {
                return Ok(Token::new(TokenClass::CHARLITERAL, self.escape_map.get(data.as_str()).unwrap(), line, column));
            }

            self.error(c, line, column);
            return Ok(Token::new(TokenClass::INVALID, "", line, column));
        }

        if c == '"' {
            let mut data = String::new();
            c = unwrap_or_return!(scanner.peek());
            while c != '"' {
                data.push(c);
                unwrap_or_return!(scanner.next());
                c = unwrap_or_return!(scanner.peek());
            }
            unwrap_or_return!(scanner.next());
            return Ok(Token::new(TokenClass::STRINGLITERAL, data.as_str(), line, column));
        }

        if c.is_ascii_alphabetic() || c == '#' || c == '_' {
            let mut data = String::new();
            if c == '#'  {
                c = unwrap_or_return!(scanner.next());
                data.push(c);
                for e in "includ".chars() {
                    if c != e {
                        break;
                    } else {
                        c = unwrap_or_return!(scanner.next());
                        data.push(c);
                    }
                }

                if data.eq("include") {
                    return Ok(Token::new(TokenClass::INCLUDE, data.as_str(), line, column));
                } else {
                    self.error('#', line, column);
                    return Ok(Token::new(TokenClass::INVALID, "", line, column));
                }
            } else {
                data.push(c);
            }

            c = unwrap_or_return!(scanner.peek());
            while c.is_ascii_alphanumeric() || c == '_' {
                data.push(c);
                unwrap_or_return!(scanner.next());
                c = unwrap_or_return!(scanner.peek());
            }

            if self.key_map.contains_key(data.as_str()) {
                return Ok(Token::new(self.key_map.get(data.as_str()).unwrap().clone(), "", line, column));
            }
            return Ok(Token::new(TokenClass::IDENTIFIER, data.as_str(), line, column));
        }

        // if we reach this point, it means we did not recognise a valid token
        self.error(c, line, column);
        Ok(Token::new(TokenClass::INVALID, "", line, column))
    }
}
















