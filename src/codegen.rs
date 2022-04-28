mod expr;
mod stmt;

use crate::parser::Stmt;
use stmt::gen_stmt;

pub fn gen(ast: &[Stmt]) -> String {
    let mut asm = "    .intel_syntax noprefix
    .text
    .globl main
main:
"
    .to_string();

    for stmt in ast {
        asm.push_str(&gen_stmt(stmt));
    }
    asm.push_str("    ret\n");
    asm
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Expr;

    #[test]
    fn gen_single_stmt() {
        let ast = vec![Stmt::ExprStmt(Expr::Num(42))];
        let expected = "    .intel_syntax noprefix
    .text
    .globl main
main:
    push 42
    pop rax
    ret
";
        let actual = gen(&ast);
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_multiple_stmt() {
        let ast = vec![Stmt::ExprStmt(Expr::Num(3)), Stmt::ExprStmt(Expr::Num(42))];
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
        let actual = gen(&ast);
        assert_eq!(expected, actual);
    }
}
