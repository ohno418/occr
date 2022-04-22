#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    if let Ok(num) = input.parse() {
        return Ok(vec![Token::Num(num)]);
    };

    Err("Failed to tokenize".to_string())
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
    fn cannot_tokenize_including_space() {
        let input = "42 21";
        assert!(tokenize(input).is_err());
    }
}
