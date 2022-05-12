use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

pub struct Scanner {
    input: BufReader<File>,
    peeked: Option<char>,
    line: i32,
    column: i32,
}

impl Scanner {
    pub fn new(source: File) -> Self {
        Scanner {
            input: BufReader::new(source),
            peeked: None,
            line: 1,
            column: 0
        }
    }

    pub fn get_column(&self) -> i32 {
        self.column
    }

    pub fn get_line(&self) -> i32 {
        self.line
    }

    pub fn peek(&mut self) ->  Result<char, io::ErrorKind> {
        if let Some(c) = self.peeked {
            return Ok(c);
        }

        let mut buf = [0; 1];

        if let Err(e) = self.input.read_exact(&mut buf) {
            return Err(e.kind());
        }
        
        let r = char::from(buf[0]);
        self.peeked = Some(r);
        Ok(r)
    }

    pub fn next(&mut self) -> Result<char, io::ErrorKind> {
        let r;
        if let Some(c) = self.peeked.take() {
            r = c;
        } else {
            let mut buf = [0; 1];

            if let Err(e) = self.input.read_exact(&mut buf) {
                return Err(e.kind());
            }

            r = char::from(buf[0]);
        }

        if r == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Ok(r)
    }
}