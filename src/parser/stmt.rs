use super::expr::{parse_expr, Expr};
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr), // expression statement
}

// <stmt> ::= <expr> ";"
pub fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    let (expr, rest) = parse_expr(tokens)?;

    if let Some(Token::Punct(punct)) = rest.get(0) {
        if punct == ";" {
            return Ok((Stmt::ExprStmt(expr), &rest[1..]));
        }
    }
    Err("expected semicolon".to_string())
}
