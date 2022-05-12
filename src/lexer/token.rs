use std::fmt::{Display, Formatter};
use crate::lexer::position::Position;

#[derive(Debug)]
pub enum TokenClass {
    // the \ (backslash) is used as an escape character in the regular expression below
    // ' is used to enclose character while " is used to enclose strings

    IDENTIFIER, // ('a'|...|'z'|'A'|...|'Z'|'_')('0'|...|'9'|'a'|...|'z'|'A'|...|'Z'|'_')*

    ASSIGN, // '='

    // delimiters
    LBRA,  // '{' // left brace
    RBRA,  // '}' // right brace
    LPAR,  // '(' // left parenthesis
    RPAR,  // ')' // right parenthesis
    LSBR,  // '[' // left square brace
    RSBR,  // ']' // left square brace
    SC,    // ';' // semicolon
    COMMA, // ','

    // types
    INT,  // "int"
    VOID, // "void"
    CHAR, // "char"

    // keywords
    IF,     // "if"
    ELSE,   // "else"
    WHILE,  // "while"
    RETURN, // "return"
    STRUCT, // "struct"
    SIZEOF, // "sizeof"

    // include
    INCLUDE, // "#include"

    // literals
    STRINGLITERAL, // \".*\"  any sequence of characters enclosed within two double quote " (please be aware of the escape character backslash \)
    INTLITERAL,    // ('0'|...|'9')+
    CHARLITERAL,   // \'('a'|...|'z'|'A'|...|'Z'|'\t'|'\b'|'\n'|'\r'|'\f'|'\''|'\"'|'\\'|'\0'|'.'|','|'_'|...)\'  a character starts and end with a single quote '

    // logical operators
    LOGAND, // "&&"
    LOGOR,  // "||"

    // comparisons
    EQ, // "=="
    NE, // "!="
    LT, // '<'
    GT, // '>'
    LE, // "<="
    GE, // ">="

    // operators
    PLUS,    // '+'
    MINUS,   // '-'
    ASTERIX, // '*'  // can be used for multiplication or pointers
    DIV,     // '/'
    REM,     // '%'
    AND,     // '&'

    // struct member access
    DOT, // '.'

    // special tokens
    EOF,    // signal end of file
    INVALID, // in case we cannot recognise a character as part of a valid token

    // Object-Oriented Features
    CLASS,    // class
    EXTENDS,  // extends
    NEW       // new
}

pub struct Token {
    pub token_class: TokenClass,
    data: String,
    position: Position,
}

impl Token {
    pub fn new(token_class: TokenClass, data: &str, line_num: i32, col_num: i32) -> Self {
        Token {
            token_class,
            data: data.to_string(),
            position: Position::new(line_num, col_num),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.data.eq("") {
            write!(f, "{:?}", self.token_class)
        } else {
            write!(f, "{:?}({})", self.token_class, self.data)
        }
    }
}