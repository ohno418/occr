use super::stmt::{parse_stmt, Stmt};
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Stmt>,
}

// <function> ::= ident "(" ")" "{" <stmt>* "}"
pub fn parse_func(tokens: &[Token]) -> Result<(Function, &[Token]), String> {
    let mut rest = &tokens[..];

    // name
    let name = match rest.get(0) {
        Some(Token::Ident(name)) => name.clone(),
        _ => return Err(format!("expected a function name: {:?}", rest)),
    };
    rest = &rest[1..];

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

    Ok((Function { name, body }, rest))
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
            name: "main".to_string(),
            body: vec![Stmt::ExprStmt(Expr::Num(42))],
        };
        let (func, rest) = parse_func(&tokens).unwrap();
        assert_eq!(expected, func);
        assert_eq!(Vec::<Token>::new(), rest);
    }

    #[test]
    fn parses_multiple_stmt() {
        // hello() { 2; 3; }
        let tokens = vec![
            Token::Ident("hello".to_string()),
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
            name: "hello".to_string(),
            body: vec![Stmt::ExprStmt(Expr::Num(2)), Stmt::ExprStmt(Expr::Num(3))],
        };
        let (func, rest) = parse_func(&tokens).unwrap();
        assert_eq!(expected, func);
        assert_eq!(Vec::<Token>::new(), rest);
    }
}
