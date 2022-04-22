use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Num(u64),
}

pub fn parse(tokens: &Vec<Token>) -> Result<Node, String> {
    if tokens.len() != 1 {
        return Err("Expected only one token".to_string());
    }

    match &tokens[0] {
        Token::Num(num) => Ok(Node::Num(*num)),
    }
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
    fn cannot_parse_others() {
        let tokens = vec![Token::Num(42), Token::Num(7)];
        assert!(parse(&tokens).is_err());
    }
}
