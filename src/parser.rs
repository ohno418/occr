use crate::lexer::{OpKind, Token};

#[derive(Debug, PartialEq)]
pub enum Node {
    Num(u64),
    Add(Binary), // +
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

// <expr> ::= <add>
pub fn parse(tokens: &[Token]) -> Result<Node, String> {
    let (node, rest) = parse_add(tokens)?;

    assert!(rest.is_empty(), "extra node: {:?}", rest);

    Ok(node)
}

// <add> ::= <num> ("+" <num>)*
fn parse_add(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    let (mut node, mut rest) = parse_num(tokens)?;

    while let Some(Token::Op(OpKind::Add)) = rest.get(0) {
        let lhs = node;
        let rhs;
        (rhs, rest) = parse_num(&rest[1..])?;
        node = Node::Add(Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });
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
    fn cannot_parse_others() {
        let tokens = vec![Token::Num(42), Token::Num(7)];
        assert!(parse(&tokens).is_err());
    }
}
