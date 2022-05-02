use crate::lexer::Token;

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

    match rest.get(0) {
        Some(Token::Punct(punct)) if punct == "(" => {
            node = Expr::FnCall(Box::new(node));
            rest = &rest[1..];
            match rest.get(0) {
                Some(Token::Punct(paren_r)) if paren_r == ")" => rest = &rest[1..],
                _ => return Err(r#"expected "(""#.to_string()),
            }
        }
        _ => (),
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
            if let Some(Token::Punct(punct)) = rest.get(0) {
                if punct == ")" {
                    return Ok((node, &rest[1..]));
                }
            }
            Err("expected terminated parenthesis".to_string())
        }
        // function name
        Token::Ident(ident) => Ok((Expr::FnName(ident.clone()), &tokens[1..])),
        // number
        Token::Num(num) => Ok((Expr::Num(*num), &tokens[1..])),
        _ => Err("failed to parse primary expression".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
