use super::expr::gen_expr;
use crate::parser::{IfStruct, Stmt};
use std::sync::atomic::{AtomicUsize, Ordering};

struct LabelCounter;

impl LabelCounter {
    fn get() -> usize {
        static IDX: AtomicUsize = AtomicUsize::new(0);
        IDX.fetch_add(1, Ordering::Relaxed)
    }
}

pub fn gen_stmt(stmt: &Stmt, return_label: &str) -> Result<String, String> {
    match stmt {
        Stmt::ExprStmt(expr) => {
            let mut asm = gen_expr(expr)?;
            asm.push_str("    pop rax\n");
            Ok(asm)
        }
        Stmt::ReturnStmt(expr) => {
            let mut asm = gen_expr(expr)?;
            asm.push_str("    pop rax\n");
            asm.push_str(format!("    jmp {}\n", return_label).as_str());
            Ok(asm)
        }
        Stmt::IfStmt(if_struct) => {
            // TODO: Make labels unique.
            let IfStruct { cond, then } = &**if_struct;
            let else_label = format!(".d.if.else.{}", LabelCounter::get());
            let mut asm = gen_expr(cond)?;
            asm.push_str("    pop rax\n");
            asm.push_str("    cmp rax, 0\n");
            asm.push_str(format!("    je {}\n", else_label).as_str());
            asm.push_str(gen_stmt(then, return_label)?.as_str());
            asm.push_str(format!("{}:\n", else_label).as_str());
            Ok(asm)
        }
        Stmt::CompStmt(stmts) => {
            let mut asm = "".to_string();
            for stmt in stmts {
                asm.push_str(gen_stmt(stmt, return_label)?.as_str());
            }
            Ok(asm)
        }
        Stmt::NullStmt => Ok("".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Expr;

    #[test]
    fn gen_expr_stmt() {
        let ast = Stmt::ExprStmt(Expr::Num(42));
        let expected = "    push 42
    pop rax
";
        let actual = gen_stmt(&ast, ".d.main.return").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_return_stmt() {
        let ast = Stmt::ReturnStmt(Expr::Num(42));
        let expected = "    push 42
    pop rax
    jmp some_label
";
        let actual = gen_stmt(&ast, "some_label").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_if_stmt() {
        let ast = Stmt::IfStmt(Box::new(IfStruct {
            cond: Expr::Num(1),
            then: Stmt::ExprStmt(Expr::Num(2)),
        }));
        let expected = "    push 1
    pop rax
    cmp rax, 0
    je .d.if.else.0
    push 2
    pop rax
.d.if.else.0:
";
        let actual = gen_stmt(&ast, ".d.main.return").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_compound_stmt() {
        let ast = Stmt::CompStmt(vec![
            Stmt::ExprStmt(Expr::Num(2)),
            Stmt::ExprStmt(Expr::Num(3)),
        ]);
        let expected = "    push 2
    pop rax
    push 3
    pop rax
";
        let actual = gen_stmt(&ast, ".d.main.return").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_null_stmt() {
        let ast = Stmt::NullStmt;
        let expected = "";
        let actual = gen_stmt(&ast, ".d.main.return").unwrap();
        assert_eq!(expected, actual);
    }
}
