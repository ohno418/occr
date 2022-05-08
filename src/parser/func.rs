use super::stmt::{parse_stmt, Stmt};
use crate::lexer::{KwKind, Token};
use crate::ty::Ty;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub ty: Ty,
    pub name: String,
    pub body: Vec<Stmt>,
}

// <function> ::= <type> ident "(" ")" "{" <stmt>* "}"
pub fn parse_func(tokens: &[Token]) -> Result<(Function, &[Token]), String> {
    let mut rest = &tokens[..];

    // type
    let ty;
    (ty, rest) = parse_type(rest)?;

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

    Ok((Function { ty, name, body }, rest))
}

// <type> ::= "int"
fn parse_type(tokens: &[Token]) -> Result<(Ty, &[Token]), String> {
    if let Some(Token::Kw(KwKind::Int)) = tokens.get(0) {
        return Ok((Ty::Int, &tokens[1..]));
    }

    Err("expected a type".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::KwKind;
    use crate::parser::*;

    #[test]
    fn parses_function_with_multiple_stmt() {
        // hello() { 2; return 3; }
        let tokens = vec![
            Token::Kw(KwKind::Int),
            Token::Ident("hello".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Num(2),
            Token::Punct(";".to_string()),
            Token::Kw(KwKind::Return),
            Token::Num(3),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let expected = Function {
            ty: Ty::Int,
            name: "hello".to_string(),
            body: vec![Stmt::ExprStmt(Expr::Num(2)), Stmt::ReturnStmt(Expr::Num(3))],
        };
        let (func, rest) = parse_func(&tokens).unwrap();
        assert_eq!(expected, func);
        assert_eq!(Vec::<Token>::new(), rest);
    }
}
