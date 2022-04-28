mod expr;
mod stmt;

use crate::lexer::Token;
pub use expr::{Binary, Expr};
use stmt::parse_stmt;
pub use stmt::Stmt;

// <program> ::= <stmt>*
pub fn parse(tokens: &[Token]) -> Result<Vec<Stmt>, String> {
    let mut asm: Vec<Stmt> = vec![];
    let mut rest = tokens;
    while !rest.is_empty() {
        let stmt;
        (stmt, rest) = parse_stmt(rest)?;
        asm.push(stmt);
    }
    Ok(asm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::PunctKind;

    #[test]
    fn parses_single_num_token() {
        let tokens = vec![Token::Num(42), Token::Punct(PunctKind::Semicolon)];
        let expected = vec![Stmt::ExprStmt(Expr::Num(42))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_add_expr() {
        let tokens = vec![
            Token::Num(12),
            Token::Punct(PunctKind::Add),
            Token::Num(23),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Num(12)),
            rhs: Box::new(Expr::Num(23)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_nested_add_expr() {
        let tokens = vec![
            Token::Num(12),
            Token::Punct(PunctKind::Add),
            Token::Num(23),
            Token::Punct(PunctKind::Add),
            Token::Num(34),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(12)),
                rhs: Box::new(Expr::Num(23)),
            })),
            rhs: Box::new(Expr::Num(34)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_sub_expr() {
        let tokens = vec![
            Token::Num(23),
            Token::Punct(PunctKind::Sub),
            Token::Num(12),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Num(23)),
            rhs: Box::new(Expr::Num(12)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_mul_expr() {
        let tokens = vec![
            Token::Num(2),
            Token::Punct(PunctKind::Mul),
            Token::Num(3),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Mul(Binary {
            lhs: Box::new(Expr::Num(2)),
            rhs: Box::new(Expr::Num(3)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_mul_expr_with_add() {
        // 1+2*3-4
        let tokens = vec![
            Token::Num(1),
            Token::Punct(PunctKind::Add),
            Token::Num(2),
            Token::Punct(PunctKind::Mul),
            Token::Num(3),
            Token::Punct(PunctKind::Sub),
            Token::Num(4),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Mul(Binary {
                    lhs: Box::new(Expr::Num(2)),
                    rhs: Box::new(Expr::Num(3)),
                })),
            })),
            rhs: Box::new(Expr::Num(4)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_div_expr_with_add() {
        // 1+3/2-4
        let tokens = vec![
            Token::Num(1),
            Token::Punct(PunctKind::Add),
            Token::Num(3),
            Token::Punct(PunctKind::Div),
            Token::Num(2),
            Token::Punct(PunctKind::Sub),
            Token::Num(4),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Div(Binary {
                    lhs: Box::new(Expr::Num(3)),
                    rhs: Box::new(Expr::Num(2)),
                })),
            })),
            rhs: Box::new(Expr::Num(4)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_expr_without_parenthesis() {
        // 1+2*3
        let tokens = vec![
            Token::Num(1),
            Token::Punct(PunctKind::Add),
            Token::Num(2),
            Token::Punct(PunctKind::Mul),
            Token::Num(3),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Num(1)),
            rhs: Box::new(Expr::Mul(Binary {
                lhs: Box::new(Expr::Num(2)),
                rhs: Box::new(Expr::Num(3)),
            })),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_expr_with_parenthesis() {
        // (1+2)*3
        let tokens = vec![
            Token::Punct(PunctKind::ParenL),
            Token::Num(1),
            Token::Punct(PunctKind::Add),
            Token::Num(2),
            Token::Punct(PunctKind::ParenR),
            Token::Punct(PunctKind::Mul),
            Token::Num(3),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Mul(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Num(2)),
            })),
            rhs: Box::new(Expr::Num(3)),
        }))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_multiple_stmt() {
        let tokens = vec![
            Token::Num(2),
            Token::Punct(PunctKind::Semicolon),
            Token::Num(3),
            Token::Punct(PunctKind::Semicolon),
        ];
        let expected = vec![Stmt::ExprStmt(Expr::Num(2)), Stmt::ExprStmt(Expr::Num(3))];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_others() {
        let tokens = vec![
            Token::Num(42),
            Token::Num(7),
            Token::Punct(PunctKind::Semicolon),
        ];
        assert!(parse(&tokens).is_err());
    }
}
