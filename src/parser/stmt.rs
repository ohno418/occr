use super::expr::{parse_expr, Expr};
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr), // expression statement
    NullStmt,       // null statement
}

// <stmt> ::= ";"
//          | <expr> ";"
pub fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    // null statement
    if let Some(Token::Punct(punct)) = tokens.get(0) {
        if punct == ";" {
            return Ok((Stmt::NullStmt, &tokens[1..]));
        }
    }

    // expression statement
    let (expr, rest) = parse_expr(tokens)?;

    if let Some(Token::Punct(punct)) = rest.get(0) {
        if punct == ";" {
            return Ok((Stmt::ExprStmt(expr), &rest[1..]));
        }
    }
    Err("expected semicolon".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null_stmt() {
        let tokens = vec![Token::Punct(";".to_string())];
        let expected = Stmt::NullStmt;
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

}
