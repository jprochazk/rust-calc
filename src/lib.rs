pub mod error;
pub mod expr;
pub mod folder;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod token;

pub mod alloc_exact_stack;
pub mod register;
pub mod rpn;
pub mod stack;
pub mod stack_pointer;
pub mod unsafe_register;
pub mod unsafe_stack;

pub type Result<T = i64, E = error::Error> = std::result::Result<T, E>;
