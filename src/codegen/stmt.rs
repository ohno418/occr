use super::expr::gen_expr;
use crate::parser::Stmt;

pub fn gen_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::ExprStmt(expr) => {
            let mut asm = gen_expr(expr);
            asm.push_str("    pop rax\n");
            asm
        }
        Stmt::NullStmt => "".to_string(),
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
        let actual = gen_stmt(&ast);
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_null_stmt() {
        let ast = Stmt::NullStmt;
        let expected = "";
        let actual = gen_stmt(&ast);
        assert_eq!(expected, actual);
    }
}
