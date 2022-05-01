mod expr;
mod stmt;

use crate::lexer::Token;
pub use expr::{Binary, Expr};
use stmt::parse_stmt;
pub use stmt::Stmt;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub body: Vec<Stmt>,
}

// <program> ::= <function>
pub fn parse(tokens: &[Token]) -> Result<Function, String> {
    let (func, rest) = parse_func(tokens)?;

    if rest.is_empty() {
        Ok(func)
    } else {
        Err(format!("extra token: {:?}", rest))
    }
}

// <function> ::= "main" "(" ")" "{" <stmt>* "}"
fn parse_func(tokens: &[Token]) -> Result<(Function, &[Token]), String> {
    let mut rest = &tokens[..];

    // name
    if let Some(Token::Ident(fn_name)) = rest.get(0) {
        if fn_name != "main" {
            return Err(format!(r#"expected "main", but get {}"#, fn_name));
        }
        rest = &rest[1..];
    }

    if let Some(Token::Punct(p)) = rest.get(0) {
        if p != "(" {
            return Err(format!(r#"expected "(", but get {}"#, p));
        }
        rest = &rest[1..];
    }
    if let Some(Token::Punct(p)) = rest.get(0) {
        if p != ")" {
            return Err(format!(r#"expected ")", but get {}"#, p));
        }
        rest = &rest[1..];
    }
    if let Some(Token::Punct(p)) = rest.get(0) {
        if p != "{" {
            return Err(format!(r#"expected "{{", but get {}"#, p));
        }
        rest = &rest[1..];
    }

    // body
    let mut body: Vec<Stmt> = Vec::new();
    loop {
        let stmt;
        (stmt, rest) = parse_stmt(rest)?;
        body.push(stmt);

        if let Some(Token::Punct(p)) = rest.get(0) {
            if p == "}" {
                rest = &rest[1..];
                break;
            }
        }
    }

    Ok((Function { body }, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_main_function() {
        let tokens = vec![
            Token::Ident("main".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Num(42),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let expected = Function {
            body: vec![Stmt::ExprStmt(Expr::Num(42))],
        };
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parses_multiple_stmt() {
        let tokens = vec![
            Token::Ident("main".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Num(2),
            Token::Punct(";".to_string()),
            Token::Num(3),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let expected = Function {
            body: vec![Stmt::ExprStmt(Expr::Num(2)), Stmt::ExprStmt(Expr::Num(3))],
        };
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_others() {
        let tokens = vec![Token::Num(42), Token::Num(7), Token::Punct(";".to_string())];
        assert!(parse(&tokens).is_err());
    }
}
