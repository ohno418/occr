use super::expr::gen_expr;
use crate::parser::Stmt;

pub fn gen_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::ExprStmt(expr) => {
            let mut asm = gen_expr(expr);
            asm.push_str("    pop rax\n");
            asm
        }
    }
}
