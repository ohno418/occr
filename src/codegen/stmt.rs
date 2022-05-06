use super::expr::gen_expr;
use crate::parser::Stmt;

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
    fn gen_null_stmt() {
        let ast = Stmt::NullStmt;
        let expected = "";
        let actual = gen_stmt(&ast, ".d.main.return").unwrap();
        assert_eq!(expected, actual);
    }
}
