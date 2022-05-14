use std::fmt::{Display, Formatter};

pub struct Position {
    line: i32,
    column: i32,
}

impl Position {
    pub fn new(line: i32, column: i32) -> Self {
        Position {
            line,
            column,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.line, self.column)
    }
}