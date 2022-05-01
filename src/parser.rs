mod expr;
mod func;
mod stmt;

use crate::lexer::Token;
use func::parse_func;

pub use expr::{Binary, Expr};
pub use func::Function;
pub use stmt::Stmt;

// <program> ::= <function>
pub fn parse(tokens: &[Token]) -> Result<Function, String> {
    let (func, rest) = parse_func(tokens)?;

    if rest.is_empty() {
        Ok(func)
    } else {
        Err(format!("extra token: {:?}", rest))
    }
}
