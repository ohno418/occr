mod expr;
mod func;
mod stmt;

use crate::lexer::Token;
use func::parse_func;

pub use expr::{Binary, Expr};
pub use func::Function;
pub use stmt::Stmt;

// <program> ::= <function>*
pub fn parse(tokens: &[Token]) -> Result<Vec<Function>, String> {
    let mut funcs: Vec<Function> = Vec::new();
    let mut rest = tokens;
    while !rest.is_empty() {
        let f;
        (f, rest) = parse_func(rest)?;
        funcs.push(f);
    }
    Ok(funcs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::KwKind;
    use crate::ty::Ty;

    #[test]
    fn parses_multiple_functions() {
        // int ret() { return 42; } int main() { return 123; }
        let tokens = vec![
            Token::Kw(KwKind::Int),
            Token::Ident("ret".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Kw(KwKind::Return),
            Token::Num(42),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
            Token::Kw(KwKind::Int),
            Token::Ident("main".to_string()),
            Token::Punct("(".to_string()),
            Token::Punct(")".to_string()),
            Token::Punct("{".to_string()),
            Token::Kw(KwKind::Return),
            Token::Num(123),
            Token::Punct(";".to_string()),
            Token::Punct("}".to_string()),
        ];
        let expected = vec![
            Function {
                ty: Ty::Int,
                name: "ret".to_string(),
                body: vec![Stmt::ReturnStmt(Expr::Num(42))],
            },
            Function {
                ty: Ty::Int,
                name: "main".to_string(),
                body: vec![Stmt::ReturnStmt(Expr::Num(123))],
            },
        ];
        let actual = parse(&tokens).unwrap();
        assert_eq!(expected, actual);
    }
}
