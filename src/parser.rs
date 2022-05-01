mod expr;
mod func;
mod stmt;

use crate::lexer::Token;
use func::parse_func;

pub use expr::{Binary, Expr};
pub use func::Function;
pub use stmt::Stmt;

// <program> ::= <function>*
pub fn parse(tokens: &[Token]) -> Result<Vec<Function>, String> {
    let mut funcs: Vec<Function> = Vec::new();

    let mut rest = tokens;
    while !rest.is_empty() {
        let f;
        (f, rest) = parse_func(rest)?;
        funcs.push(f);
    }

    Ok(funcs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_functions() {
        // ret() { 42; } main() { 123; }
        let tokens = vec![
            Token::Ident("ret".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Num(42),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
            Token::Ident("main".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Num(123),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let expected = vec![
            Function {
                name: "ret".to_string(),
                body: vec![Stmt::ExprStmt(Expr::Num(42))],
            },
            Function {
                name: "main".to_string(),
                body: vec![Stmt::ExprStmt(Expr::Num(123))],
            },
        ];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }
}
