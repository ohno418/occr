use super::expr::{parse_expr, Expr};
use crate::lexer::{Token, KwKind};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),   // expression statement
    ReturnStmt(Expr), // return statement
    NullStmt,         // null statement
}

// <stmt> ::= ";"
//          | "return" <expr> ";"
//          | <expr> ";"
pub fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    // null statement
    if let Some(Token::Punct(punct)) = tokens.get(0) {
        if punct == ";" {
            return Ok((Stmt::NullStmt, &tokens[1..]));
        }
    }

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
    use crate::parser::Binary;

    #[test]
    fn parses_single_num_token() {
        let tokens = vec![Token::Num(42), Token::Punct(";".to_string())];
        let expected = Stmt::ExprStmt(Expr::Num(42));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_add_expr() {
        let tokens = vec![
            Token::Num(12),
            Token::Punct("+".to_string()),
            Token::Num(23),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Num(12)),
            rhs: Box::new(Expr::Num(23)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_nested_add_expr() {
        let tokens = vec![
            Token::Num(12),
            Token::Punct("+".to_string()),
            Token::Num(23),
            Token::Punct("+".to_string()),
            Token::Num(34),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(12)),
                rhs: Box::new(Expr::Num(23)),
            })),
            rhs: Box::new(Expr::Num(34)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_sub_expr() {
        let tokens = vec![
            Token::Num(23),
            Token::Punct("-".to_string()),
            Token::Num(12),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Num(23)),
            rhs: Box::new(Expr::Num(12)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_mul_expr() {
        let tokens = vec![
            Token::Num(2),
            Token::Punct("*".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Mul(Binary {
            lhs: Box::new(Expr::Num(2)),
            rhs: Box::new(Expr::Num(3)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_mul_expr_with_add() {
        // 1+2*3-4
        let tokens = vec![
            Token::Num(1),
            Token::Punct("+".to_string()),
            Token::Num(2),
            Token::Punct("*".to_string()),
            Token::Num(3),
            Token::Punct("-".to_string()),
            Token::Num(4),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Mul(Binary {
                    lhs: Box::new(Expr::Num(2)),
                    rhs: Box::new(Expr::Num(3)),
                })),
            })),
            rhs: Box::new(Expr::Num(4)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_div_expr_with_add() {
        // 1+3/2-4
        let tokens = vec![
            Token::Num(1),
            Token::Punct("+".to_string()),
            Token::Num(3),
            Token::Punct("/".to_string()),
            Token::Num(2),
            Token::Punct("-".to_string()),
            Token::Num(4),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Div(Binary {
                    lhs: Box::new(Expr::Num(3)),
                    rhs: Box::new(Expr::Num(2)),
                })),
            })),
            rhs: Box::new(Expr::Num(4)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_expr_without_parenthesis() {
        // 1+2*3
        let tokens = vec![
            Token::Num(1),
            Token::Punct("+".to_string()),
            Token::Num(2),
            Token::Punct("*".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Num(1)),
            rhs: Box::new(Expr::Mul(Binary {
                lhs: Box::new(Expr::Num(2)),
                rhs: Box::new(Expr::Num(3)),
            })),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_expr_with_parenthesis() {
        // (1+2)*3
        let tokens = vec![
            Token::Punct("(".to_string()),
            Token::Num(1),
            Token::Punct("+".to_string()),
            Token::Num(2),
            Token::Punct(")".to_string()),
            Token::Punct("*".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
        ];
        let expected = Stmt::ExprStmt(Expr::Mul(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Num(2)),
            })),
            rhs: Box::new(Expr::Num(3)),
        }));
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
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
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parse_null_stmt() {
        let tokens = vec![Token::Punct(";".to_string())];
        let expected = Stmt::NullStmt;
        let (stmt, rest) = parse_stmt(&tokens).unwrap();
        assert_eq!(expected, stmt);
        assert_eq!(Vec::<Token>::new(), rest);
    }
}
