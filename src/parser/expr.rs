use crate::lexer::Token;
use super::consume_punct;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(u64),
    Add(Binary),       // +
    Sub(Binary),       // -
    Mul(Binary),       // *
    Div(Binary),       // *
    FnName(String),    // Function identifier
    FnCall(Box<Expr>), // function call
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

// <expr> ::= <add>
pub fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    parse_add(tokens)
}

// <add> ::= <mul> (("+" | "-") <mul>)*
fn parse_add(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (mut node, mut rest) = parse_mul(tokens)?;

    while let Some(Token::Punct(punct)) = rest.get(0) {
        if punct != "+" && punct != "-" {
            break;
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_mul(&rest[1..])?;

        let bin = Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        node = if punct == "+" {
            Expr::Add(bin)
        } else if punct == "-" {
            Expr::Sub(bin)
        } else {
            unreachable!()
        };
    }

    Ok((node, rest))
}

// <mul> ::= <postfix> ("*" <postfix>)*
fn parse_mul(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (mut node, mut rest) = parse_postfix(tokens)?;

    while let Some(Token::Punct(punct)) = rest.get(0) {
        if punct != "*" && punct != "/" {
            break;
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_postfix(&rest[1..])?;

        let bin = Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        node = if punct == "*" {
            Expr::Mul(bin)
        } else if punct == "/" {
            Expr::Div(bin)
        } else {
            unreachable!()
        };
    }

    Ok((node, rest))
}

// <postfix> ::= <primary> ("(" ")")?
fn parse_postfix(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (mut node, mut rest) = parse_primary(tokens)?;

    match consume_punct(rest, "(") {
        Ok(r) => {
            node = Expr::FnCall(Box::new(node));
            rest = consume_punct(r, ")")?;
        }
        Err(_) => (),
    }

    Ok((node, rest))
}

// <primary> ::= "(" <expr> ")"
//             | func-name
//             | number
fn parse_primary(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    match tokens.get(0).expect("expected some primary expression") {
        // "(" <expr> ")"
        Token::Punct(punct) if punct == "(" => {
            let (node, rest) = parse_expr(&tokens[1..])?;
            Ok((node, consume_punct(rest, ")")?))
        }
        // function name
        Token::Ident(ident) => {
            // TODO: Check if the function exists.
            Ok((Expr::FnName(ident.clone()), &tokens[1..]))
        }
        // number
        Token::Num(num) => Ok((Expr::Num(*num), &tokens[1..])),
        _ => Err("failed to parse primary expression".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_num_token() {
        let tokens = vec![Token::Num(42)];
        let expected = Expr::Num(42);
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_add_expr() {
        let tokens = vec![
            Token::Num(12),
            Token::Punct("+".to_string()),
            Token::Num(23),
        ];
        let expected = Expr::Add(Binary {
            lhs: Box::new(Expr::Num(12)),
            rhs: Box::new(Expr::Num(23)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_function_call() {
        let tokens = vec![
            Token::Ident("somefunc".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
        ];
        let expected = Expr::FnCall(Box::new(Expr::FnName("somefunc".to_string())));
        let (expr, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, expr);
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
        ];
        let expected = Expr::Add(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(12)),
                rhs: Box::new(Expr::Num(23)),
            })),
            rhs: Box::new(Expr::Num(34)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_sub_expr() {
        let tokens = vec![
            Token::Num(23),
            Token::Punct("-".to_string()),
            Token::Num(12),
        ];
        let expected = Expr::Sub(Binary {
            lhs: Box::new(Expr::Num(23)),
            rhs: Box::new(Expr::Num(12)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_mul_expr() {
        let tokens = vec![Token::Num(2), Token::Punct("*".to_string()), Token::Num(3)];
        let expected = Expr::Mul(Binary {
            lhs: Box::new(Expr::Num(2)),
            rhs: Box::new(Expr::Num(3)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
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
        ];
        let expected = Expr::Sub(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Mul(Binary {
                    lhs: Box::new(Expr::Num(2)),
                    rhs: Box::new(Expr::Num(3)),
                })),
            })),
            rhs: Box::new(Expr::Num(4)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
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
        ];
        let expected = Expr::Sub(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Div(Binary {
                    lhs: Box::new(Expr::Num(3)),
                    rhs: Box::new(Expr::Num(2)),
                })),
            })),
            rhs: Box::new(Expr::Num(4)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
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
        ];
        let expected = Expr::Add(Binary {
            lhs: Box::new(Expr::Num(1)),
            rhs: Box::new(Expr::Mul(Binary {
                lhs: Box::new(Expr::Num(2)),
                rhs: Box::new(Expr::Num(3)),
            })),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
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
        ];
        let expected = Expr::Mul(Binary {
            lhs: Box::new(Expr::Add(Binary {
                lhs: Box::new(Expr::Num(1)),
                rhs: Box::new(Expr::Num(2)),
            })),
            rhs: Box::new(Expr::Num(3)),
        });
        let (actual, rest) = parse_expr(&tokens).unwrap();
        assert_eq!(expected, actual);
        assert_eq!(Vec::<Token>::new(), rest);
    }
}
