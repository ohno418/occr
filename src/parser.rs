use crate::lexer::{PunctKind, Token};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr), // expression statement
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(u64),
    Add(Binary), // +
    Sub(Binary), // -
    Mul(Binary), // *
    Div(Binary), // *
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

// <program> ::= <stmt>
pub fn parse(tokens: &[Token]) -> Result<Stmt, String> {
    let (node, rest) = parse_stmt(tokens)?;
    if !rest.is_empty() {
        return Err(format!("extra token: {:?}", rest));
    }
    Ok(node)
}

// <stmt> ::= <expr> ";"
fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), String> {
    let (expr, rest) = parse_expr(tokens)?;

    if let Some(Token::Punct(PunctKind::Semicolon)) = rest.get(0) {
        Ok((Stmt::ExprStmt(expr), &rest[1..]))
    } else {
        Err("expected semicolon".to_string())
    }
}

// <expr> ::= <add>
fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    parse_add(tokens)
}

// <add> ::= <mul> (("+" | "-") <mul>)*
fn parse_add(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (mut node, mut rest) = parse_mul(tokens)?;

    while let Some(Token::Punct(punct_kind)) = rest.get(0) {
        match punct_kind {
            PunctKind::Add | PunctKind::Sub => (),
            _ => break,
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_mul(&rest[1..])?;

        let bin = Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        node = match punct_kind {
            PunctKind::Add => Expr::Add(bin),
            PunctKind::Sub => Expr::Sub(bin),
            _ => unreachable!(),
        };
    }

    Ok((node, rest))
}

// <mul> ::= <primary> ("*" <primary>)*
fn parse_mul(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (mut node, mut rest) = parse_primary(tokens)?;

    while let Some(Token::Punct(punct_kind)) = rest.get(0) {
        match punct_kind {
            PunctKind::Mul | PunctKind::Div => (),
            _ => break,
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_primary(&rest[1..])?;

        let bin = Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        node = match punct_kind {
            PunctKind::Mul => Expr::Mul(bin),
            PunctKind::Div => Expr::Div(bin),
            _ => unreachable!(),
        };
    }

    Ok((node, rest))
}

// <primary> ::= "(" <expr> ")"
//             | number
fn parse_primary(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    match tokens.get(0).expect("expected some primary expression") {
        Token::Punct(PunctKind::ParenL) => {
            let (node, rest) = parse_expr(&tokens[1..])?;
            if let Some(Token::Punct(PunctKind::ParenR)) = rest.get(0) {
                Ok((node, &rest[1..]))
            } else {
                Err("expected terminated parenthesis".to_string())
            }
        }
        Token::Num(num) => Ok((Expr::Num(*num), &tokens[1..])),
        _ => Err("failed to parse primary expression".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_num_token() {
        let tokens = vec![Token::Num(42), Token::Punct(PunctKind::Semicolon)];
        let expected = Stmt::ExprStmt(Expr::Num(42));
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
        let expected = Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Num(12)),
            rhs: Box::new(Expr::Num(23)),
        }));
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
        let expected = Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(12)),
                rhs: Box::new(Expr::Num(23)),
            })),
            rhs: Box::new(Expr::Num(34)),
        }));
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
        let expected = Stmt::ExprStmt(Expr::Sub(Binary {
            lhs: Box::new(Expr::Num(23)),
            rhs: Box::new(Expr::Num(12)),
        }));
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
        let expected = Stmt::ExprStmt(Expr::Mul(Binary {
            lhs: Box::new(Expr::Num(2)),
            rhs: Box::new(Expr::Num(3)),
        }));
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
        let expected = Stmt::ExprStmt(Expr::Add(Binary {
            lhs: Box::new(Expr::Num(1)),
            rhs: Box::new(Expr::Mul(Binary {
                lhs: Box::new(Expr::Num(2)),
                rhs: Box::new(Expr::Num(3)),
            })),
        }));
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
        let expected = Stmt::ExprStmt(Expr::Mul(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Num(2)),
            })),
            rhs: Box::new(Expr::Num(3)),
        }));
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
