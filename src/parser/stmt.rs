use super::expr::{parse_expr, Expr};
use crate::lexer::{KwKind, Token};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),      // expression statement
    ReturnStmt(Expr),    // return statement
    CompStmt(Vec<Stmt>), // compound statement (block)
    NullStmt,            // null statement
}

// <stmt> ::= ";"
//          | "return" <expr> ";"
//          | "{" <stmt>* "}"
//          | <expr> ";"
pub fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    // return statement
    if let Some(Token::Kw(KwKind::Return)) = tokens.get(0) {
        let (expr, rest) = parse_expr(&tokens[1..])?;
        if let Some(Token::Punct(punct)) = rest.get(0) {
            if punct == ";" {
                return Ok((Stmt::ReturnStmt(expr), &rest[1..]));
            }
        }
        return Err("expected semicolon".to_string());
    }

    if let Some(Token::Punct(punct)) = tokens.get(0) {
        match punct.as_str() {
            // null statement
            ";" => return Ok((Stmt::NullStmt, &tokens[1..])),
            // compound statement
            "{" => {
                let mut stmts: Vec<Stmt> = Vec::new();
                let mut rest = &tokens[1..];
                loop {
                    if let Some(Token::Punct(punct)) = rest.get(0) {
                        if punct == "}" {
                            rest = &rest[1..];
                            break;
                        }
                    }

                    let stmt;
                    (stmt, rest) = parse_stmt(rest)?;
                    stmts.push(stmt);
                }
                return Ok((Stmt::CompStmt(stmts), rest));
            }
            _ => (),
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
    fn parse_expression_stmt() {
        let tokens = vec![Token::Num(42), Token::Punct(";".to_string())];
        let expected = Stmt::ExprStmt(Expr::Num(42));
        let (actual, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parse_return_stmt() {
        let tokens = vec![
            Token::Kw(KwKind::Return),
            Token::Num(42),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ReturnStmt(Expr::Num(42));
        let (actual, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parse_compound_stmt() {
        // { 2; 3; 4; }
        let tokens = vec![
            Token::Punct("{".to_string()),
            Token::Num(2),
            Token::Punct(";".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
            Token::Num(4),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let expected = Stmt::CompStmt(vec![
            Stmt::ExprStmt(Expr::Num(2)),
            Stmt::ExprStmt(Expr::Num(3)),
            Stmt::ExprStmt(Expr::Num(4)),
        ]);
        let (actual, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parse_null_stmt() {
        let tokens = vec![Token::Punct(";".to_string())];
        let expected = Stmt::NullStmt;
        let (actual, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }
}
