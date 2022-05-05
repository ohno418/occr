mod expr;
mod stmt;

use crate::parser::Function;
use stmt::gen_stmt;

pub fn gen(ast: &[Function]) -> Result<String, String> {
    let mut asm = "    .intel_syntax noprefix
    .text
    .globl main
"
    .to_string();

    for func in ast {
        asm.push_str(&format!("{}:\n", func.name));
        for stmt in &func.body {
            asm.push_str(&gen_stmt(&stmt)?);
        }
        asm.push_str("    ret\n");
    }
    Ok(asm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Expr, Stmt};

    #[test]
    fn gen_single_stmt() {
        let ast = vec![Function {
            name: "main".to_string(),
            body: vec![Stmt::ExprStmt(Expr::Num(42))],
        }];
        let expected = "    .intel_syntax noprefix
    .text
    .globl main
main:
    push 42
    pop rax
    ret
";
        let actual = gen(&ast).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_multiple_stmt() {
        let ast = vec![Function {
            name: "main".to_string(),
            body: vec![Stmt::ExprStmt(Expr::Num(3)), Stmt::ExprStmt(Expr::Num(42))],
        }];
        let expected = "    .intel_syntax noprefix
    .text
    .globl main
main:
    push 3
    pop rax
    push 42
    pop rax
    ret
";
        let actual = gen(&ast).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_multiple_functions() {
        // ret() { 42; } main() { 123; }
        let ast = vec![
            Function {
                name: "ret".to_string(),
                body: vec![Stmt::ExprStmt(Expr::Num(42))],
            },
            Function {
                name: "main".to_string(),
                body: vec![Stmt::ExprStmt(Expr::Num(123))],
            },
        ];
        let expected = "    .intel_syntax noprefix
    .text
    .globl main
ret:
    push 42
    pop rax
    ret
main:
    push 123
    pop rax
    ret
";
        let actual = gen(&ast).unwrap();
        assert_eq!(expected, actual);
    }
}
