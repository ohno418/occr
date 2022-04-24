#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut rest = input;
    loop {
        let c = match rest.chars().next() {
            Some(c) => c,
            None => break,
        };

        // skip whitespace
        if c == ' ' {
            rest = &rest[1..];
            continue;
        }

        // number
        if c.is_digit(10) {
            let num;
            (num, rest) = take_number_from_start(&rest).expect("failed to take number");
            tokens.push(Token::Num(num));
            continue;
        }

        return Err(format!("Failed to tokenize: {}", c));
    }

    Ok(tokens)
}

// Takes a number from the start of `s`, and returns the rest of the str.
//
// e.g.
//   take_number_from_start("123hello") => Some(123, "hello")
//   take_number_from_start("hello123") => None
fn take_number_from_start<'a>(s: &'a str) -> Option<(u64, &'a str)> {
    let mut num_str = "".to_string();
    let mut chars = s.chars();
    let mut rest = s;
    loop {
        if let Some(c) = chars.next() {
            if c.is_digit(10) {
                num_str.push(c);
                rest = &rest[1..];
                continue;
            };
        };
        break;
    }
    match num_str.parse() {
        Ok(num) => Some((num, rest)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_single_digit_number() {
        let input = "2";
        let expected = vec![Token::Num(2)];
        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_multi_digit_number() {
        let input = "123";
        let expected = vec![Token::Num(123)];
        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_with_spaces() {
        let input = "  42 ";
        let expected = vec![Token::Num(42)];
        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_tokenize_string() {
        let input = " 42  hi";
        assert!(tokenize(input).is_err());
    }

    mod tests_take_number_from {
        use super::take_number_from_start;

        #[test]
        fn takes_number_from_the_start() {
            let s = "123hello";
            assert_eq!(take_number_from_start(s), Some((123, "hello")));
        }

        #[test]
        fn returns_none_for_not_starting_with_number() {
            let s = "hello123";
            assert_eq!(take_number_from_start(s), None);
        }
    }
}
