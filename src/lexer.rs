#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // TODO: Can improve with a closure?
    let mut tokens: Vec<Token> = Vec::new();
    for word in input.split_whitespace() {
        if let Ok(num) = word.parse() {
            tokens.push(Token::Num(num));
        } else {
            return Err("Failed to tokenize".to_string())
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_single_number() {
        let input = "42";
        let expected = vec![Token::Num(42)];
        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_single_number_with_spaces() {
        let input = "  42 ";
        let expected = vec![Token::Num(42)];
        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_numbers() {
        let input = " 42  21";
        let expected = vec![Token::Num(42), Token::Num(21)];
        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_tokenize_string() {
        let input = " 42  hi";
        assert!(tokenize(input).is_err());
    }
}
