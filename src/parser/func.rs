use super::stmt::{parse_stmt, Stmt};
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub body: Vec<Stmt>,
}

// <function> ::= "main" "(" ")" "{" <stmt>* "}"
pub fn parse_func(tokens: &[Token]) -> Result<(Function, &[Token]), String> {
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
    use crate::parser::*;

    #[test]
    fn parses_main_function() {
        // main() { 42; }
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
        let (func, rest) = parse_func(&tokens).unwrap();
        assert_eq!(expected, func);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_multiple_stmt() {
        // main() { 2; 3; }
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
        let (func, rest) = parse_func(&tokens).unwrap();
        assert_eq!(expected, func);
        assert_eq!(Vec::<Token>::new(), rest);
    }
}
