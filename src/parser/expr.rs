use crate::lexer::Token;

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

// <mul> ::= <primary> ("*" <primary>)*
fn parse_mul(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    let (mut node, mut rest) = parse_primary(tokens)?;

    while let Some(Token::Punct(punct)) = rest.get(0) {
        if punct != "*" && punct != "/" {
            break;
        }

        let lhs = node;
        let rhs;
        (rhs, rest) = parse_primary(&rest[1..])?;

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

// <primary> ::= "(" <expr> ")"
//             | number
fn parse_primary(tokens: &[Token]) -> Result<(Expr, &[Token]), String> {
    match tokens.get(0).expect("expected some primary expression") {
        Token::Punct(punct) if punct == "(" => {
            let (node, rest) = parse_expr(&tokens[1..])?;
            if let Some(Token::Punct(punct)) = rest.get(0) {
                if punct == ")" {
                    return Ok((node, &rest[1..]));
                }
            }
            Err("expected terminated parenthesis".to_string())
        }
        Token::Num(num) => Ok((Expr::Num(*num), &tokens[1..])),
        _ => Err("failed to parse primary expression".to_string()),
    }
}
