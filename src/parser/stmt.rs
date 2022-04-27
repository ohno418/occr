use super::expr::{parse_expr, Expr};
use crate::lexer::{PunctKind, Token};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr), // expression statement
}

// <stmt> ::= <expr> ";"
pub fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    let (expr, rest) = parse_expr(tokens)?;

    if let Some(Token::Punct(PunctKind::Semicolon)) = rest.get(0) {
        Ok((Stmt::ExprStmt(expr), &rest[1..]))
    } else {
        Err("expected semicolon".to_string())
    }
}
