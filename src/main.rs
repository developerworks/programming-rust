use std::fmt::Display;

use thiserror::Error;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub struct JsonError {
    message: String,
    line: usize,
    column: usize,
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

#[derive(Debug, Error)]
#[error("{message:} ({line:}, {column:})")]
pub struct TestError {
    message: String,
    line: usize,
    column: usize
}
