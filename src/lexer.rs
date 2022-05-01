#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u64),
    // puctuator
    Punct(String),
    // identifier
    Ident(String),
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
        if c.is_ascii_digit() {
            let num;
            (num, rest) = take_number_from_start(&rest).expect("failed to take number");
            tokens.push(Token::Num(num));
            continue;
        }

        // operator
        if c.is_ascii_punctuation() {
            match c {
                '+' | '-' | '*' | '/' | '(' | ')' | '{' | '}' | ';' => {
                    tokens.push(Token::Punct(c.to_string()));
                    rest = &rest[1..];
                    continue;
                }
                _ => return Err(format!("unknown punctuator: {}", c)),
            }
        }

        // identifier
        if c.is_ascii_alphabetic() {
            let ident;
            (ident, rest) = match take_ident_from_start(rest) {
                Some((ident, rest)) => (ident, rest),
                None => return Err("identifier not found".to_string()),
            };
            tokens.push(Token::Ident(ident.to_string()));
            continue;
        }

        return Err(format!("unexpected input: {}", rest));
    }

    Ok(tokens)
}

// Takes a number from the start of `s`, and returns the rest of the str.
//
// e.g.
//   take_number_from_start("123hello") => Some(123, "hello")
//   take_number_from_start("hello123") => None
fn take_number_from_start<'a>(s: &'a str) -> Option<(u64, &'a str)> {
    let mut len = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            len += 1;
        } else {
            break;
        }
    }

    match len {
        0 => None,
        _ => Some((
            (&s[..len])
                .parse()
                .expect(format!(r#"failed to parse "{}" into number"#, &s[..len]).as_str()),
            &s[len..],
        )),
    }
}

// Takes an ident from the start of `s`, and returns the rest of the str.
//
// e.g.
//   take_ident_from_start("hello123") => Some(("hello", "123"))
//   take_ident_from_start("123hello") => None
fn take_ident_from_start<'a>(s: &'a str) -> Option<(&'a str, &'a str)> {
    let mut len = 0;
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            len += 1;
        } else {
            break;
        }
    }

    match len {
        0 => None,
        _ => Some((&s[..len], &s[len..])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_single_digit_number() {
        let input = "2;";
        let expected = vec![Token::Num(2), Token::Punct(";".to_string())];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_multi_digit_number() {
        let input = "123;";
        let expected = vec![Token::Num(123), Token::Punct(";".to_string())];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_with_spaces() {
        let input = "  42 ;";
        let expected = vec![Token::Num(42), Token::Punct(";".to_string())];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_add_expr() {
        let input = "12+23;";
        let expected = vec![
            Token::Num(12),
            Token::Punct("+".to_string()),
            Token::Num(23),
            Token::Punct(";".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_sub_expr() {
        let input = "23-12;";
        let expected = vec![
            Token::Num(23),
            Token::Punct("-".to_string()),
            Token::Num(12),
            Token::Punct(";".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_mul_expr() {
        let input = "2*3;";
        let expected = vec![
            Token::Num(2),
            Token::Punct("*".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_div_expr() {
        let input = "9/3;";
        let expected = vec![
            Token::Num(9),
            Token::Punct("/".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_expr_with_parenthesis() {
        let input = "(1+2)*3;";
        let expected = vec![
            Token::Punct("(".to_string()),
            Token::Num(1),
            Token::Punct("+".to_string()),
            Token::Num(2),
            Token::Punct(")".to_string()),
            Token::Punct("*".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tokenizes_with_identifier() {
        let input = "main() { 42; }";
        let expected = vec![
            Token::Ident("main".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Num(42),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let actual = tokenize(input).unwrap();
        assert_eq!(expected, actual);
    }

    mod tests_take_number_from_start {
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

    mod tests_take_ident_from_start {
        use super::take_ident_from_start;

        #[test]
        fn takes_identifier_from_the_start() {
            let s = "hello123";
            assert_eq!(take_ident_from_start(s), Some(("hello", "123")));
        }

        #[test]
        fn returns_none_for_not_starting_with_identifier() {
            let s = "123hello";
            assert_eq!(take_ident_from_start(s), None);
        }
    }
}
