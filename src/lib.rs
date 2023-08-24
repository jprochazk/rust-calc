pub mod compiler;
pub mod error;
pub mod expr;
pub mod folder;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod span;
pub mod token;
pub mod vm;

pub type Result<T = i64, E = error::Error> = std::result::Result<T, E>;
