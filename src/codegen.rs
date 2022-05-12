mod expr;
mod stmt;

use crate::parser::Function;
use stmt::gen_stmt;

pub fn gen(ast: &[Function]) -> Result<String, String> {
    let mut label_index = LabelIndex::new();

    let mut asm = "    .intel_syntax noprefix
    .text
    .globl main
"
    .to_string();

    for func in ast {
        let return_label = format!(".d.{}.return", func.name);

        asm.push_str(&format!("{}:\n", func.name));
        for stmt in &func.body {
            asm.push_str(&gen_stmt(&stmt, &return_label, &mut label_index)?);
        }
        asm.push_str(format!("{}:\n", return_label).as_str());
        asm.push_str("    ret\n");
    }
    Ok(asm)
}

// This provides an index number to a label to make it globally unique.
struct LabelIndex(u64);

impl LabelIndex {
    fn new() -> Self {
        Self(0)
    }

    fn get(&mut self) -> u64 {
        let prev = self.0;
        self.0 = prev + 1;
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Expr, Stmt};
    use crate::ty::Ty;

    #[test]
    fn gen_single_stmt() {
        let ast = vec![Function {
            ty: Ty::Int,
            name: "main".to_string(),
            body: vec![Stmt::ExprStmt(Expr::Num(42))],
        }];
        let expected = "    .intel_syntax noprefix
    .text
    .globl main
main:
    push 42
    pop rax
.d.main.return:
    ret
";
        let actual = gen(&ast).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_multiple_stmt() {
        let ast = vec![Function {
            ty: Ty::Int,
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
.d.main.return:
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
                ty: Ty::Int,
                name: "ret".to_string(),
                body: vec![Stmt::ExprStmt(Expr::Num(42))],
            },
            Function {
                ty: Ty::Int,
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
.d.ret.return:
    ret
main:
    push 123
    pop rax
.d.main.return:
    ret
";
        let actual = gen(&ast).unwrap();
        assert_eq!(expected, actual);
    }
}
