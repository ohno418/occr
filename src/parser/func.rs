use super::{consume_punct, stmt::{parse_stmt, Stmt}};
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
    // type
    let (ty, rest) = parse_type(tokens)?;

    // name
    let name = match rest.get(0) {
        Some(Token::Ident(name)) => name.clone(),
        _ => return Err(format!("expected a function name: {:?}", rest)),
    };
    let rest = &rest[1..];

    let rest = consume_punct(rest, "(")?;
    let rest = consume_punct(rest, ")")?;
    let rest = consume_punct(rest, "{")?;

    // body
    let mut body: Vec<Stmt> = Vec::new();
    let mut rest = rest;
    loop {
        match consume_punct(rest, "}") {
            Ok(r) => {
                rest = r;
                break;
            }
            Err(_) => {
                let (stmt, r) = parse_stmt(rest)?;
                body.push(stmt);
                rest = r;
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
