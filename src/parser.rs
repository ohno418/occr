use crate::lexer::{OpKind, Token};

#[derive(Debug, PartialEq)]
pub enum Node {
    Num(u64),
    Add(Binary), // +
    Sub(Binary), // -
    Mul(Binary), // *
    Div(Binary), // *
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

// <expr> ::= <add>
pub fn parse(tokens: &[Token]) -> Result<Node, String> {
    let (node, rest) = parse_add(tokens)?;
    if !rest.is_empty() {
        return Err(format!("extra node: {:?}", rest));
    }
    Ok(node)
}

// <add> ::= <mul> (("+" | "-") <mul>)*
fn parse_add(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_mul(tokens)?;

    while let Some(Token::Op(op_kind)) = rest.get(0) {
        match op_kind {
            OpKind::Add | OpKind::Sub => (),
            _ => break,
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_mul(&rest[1..])?;

        let bin = Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        node = match op_kind {
            OpKind::Add => Node::Add(bin),
            OpKind::Sub => Node::Sub(bin),
            _ => unreachable!(),
        };
    }

    Ok((node, rest))
}

// <mul> ::= <num> ("*" <num>)*
fn parse_mul(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_num(tokens)?;

    while let Some(Token::Op(op_kind)) = rest.get(0) {
        match op_kind {
            OpKind::Mul | OpKind::Div => (),
            _ => break,
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_num(&rest[1..])?;

        let bin = Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        node = match op_kind {
            OpKind::Mul => Node::Mul(bin),
            OpKind::Div => Node::Div(bin),
            _ => unreachable!(),
        };
    }

    Ok((node, rest))
}

// <num> ::= number
fn parse_num(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    if let Some(Token::Num(num)) = tokens.get(0) {
        return Ok((Node::Num(*num), &tokens[1..]));
    }

    Err("failed to parse number".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_num_token() {
        let tokens = vec![Token::Num(42)];
        let expected = Node::Num(42);
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_add_expr() {
        let tokens = vec![Token::Num(12), Token::Op(OpKind::Add), Token::Num(23)];
        let expected = Node::Add(Binary {
            lhs: Box::new(Node::Num(12)),
            rhs: Box::new(Node::Num(23)),
        });
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_nested_add_expr() {
        let tokens = vec![
            Token::Num(12),
            Token::Op(OpKind::Add),
            Token::Num(23),
            Token::Op(OpKind::Add),
            Token::Num(34),
        ];
        let expected = Node::Add(Binary {
            lhs: Box::new(Node::Add(Binary {
                lhs: Box::new(Node::Num(12)),
                rhs: Box::new(Node::Num(23)),
            })),
            rhs: Box::new(Node::Num(34)),
        });
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_sub_expr() {
        let tokens = vec![Token::Num(23), Token::Op(OpKind::Sub), Token::Num(12)];
        let expected = Node::Sub(Binary {
            lhs: Box::new(Node::Num(23)),
            rhs: Box::new(Node::Num(12)),
        });
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_mul_expr() {
        let tokens = vec![Token::Num(2), Token::Op(OpKind::Mul), Token::Num(3)];
        let expected = Node::Mul(Binary {
            lhs: Box::new(Node::Num(2)),
            rhs: Box::new(Node::Num(3)),
        });
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_mul_expr_with_add() {
        // 1+2*3-4
        let tokens = vec![Token::Num(1), Token::Op(OpKind::Add), Token::Num(2), Token::Op(OpKind::Mul), Token::Num(3), Token::Op(OpKind::Sub), Token::Num(4)];
        let expected =
            Node::Sub(Binary {
                lhs: Box::new(Node::Add(Binary {
                    lhs: Box::new(Node::Num(1)),
                    rhs: Box::new(Node::Mul(Binary {
                        lhs: Box::new(Node::Num(2)),
                        rhs: Box::new(Node::Num(3)),
                    })),
                })),
                rhs: Box::new(Node::Num(4)),
            });
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_div_expr_with_add() {
        // 1+3/2-4
        let tokens = vec![Token::Num(1), Token::Op(OpKind::Add), Token::Num(3), Token::Op(OpKind::Div), Token::Num(2), Token::Op(OpKind::Sub), Token::Num(4)];
        let expected =
            Node::Sub(Binary {
                lhs: Box::new(Node::Add(Binary {
                    lhs: Box::new(Node::Num(1)),
                    rhs: Box::new(Node::Div(Binary {
                        lhs: Box::new(Node::Num(3)),
                        rhs: Box::new(Node::Num(2)),
                    })),
                })),
                rhs: Box::new(Node::Num(4)),
            });
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_others() {
        let tokens = vec![Token::Num(42), Token::Num(7)];
        assert!(parse(&tokens).is_err());
    }
}
