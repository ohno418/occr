use crate::parser::{Binary, Expr};

pub(super) fn gen_expr(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::Num(n) => Ok(format!("    push {}\n", n)),
        Expr::Add(bin) => {
            let Binary { lhs, rhs } = &**bin;
            let mut s = gen_expr(&lhs)?;
            s.push_str(&gen_expr(&rhs)?);
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    add rax, rdi\n");
            s.push_str("    push rax\n");
            Ok(s)
        }
        Expr::Sub(bin) => {
            let Binary { lhs, rhs } = &**bin;
            let mut s = gen_expr(&lhs)?;
            s.push_str(&gen_expr(&rhs)?);
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    sub rax, rdi\n");
            s.push_str("    push rax\n");
            Ok(s)
        }
        Expr::Mul(bin) => {
            let Binary { lhs, rhs } = &**bin;
            let mut s = gen_expr(&lhs)?;
            s.push_str(&gen_expr(&rhs)?);
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    imul rax, rdi\n");
            s.push_str("    push rax\n");
            Ok(s)
        }
        Expr::Div(bin) => {
            let Binary { lhs, rhs } = &**bin;
            let mut s = gen_expr(&lhs)?;
            s.push_str(&gen_expr(&rhs)?);
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    xor rdx, rdx\n");
            s.push_str("    idiv rdi\n");
            s.push_str("    push rax\n");
            Ok(s)
        }
        Expr::FnName(fn_name) => Err(format!("cannot generate a function: {}", fn_name)),
        Expr::FnCall(f) => {
            let fn_name = match &**f {
                Expr::FnName(fn_name) => fn_name,
                _ => return Err("expected a function identifier".to_string()),
            };
            let mut s = format!("    call {}\n", fn_name);
            s.push_str("    push rax\n");
            Ok(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_num() {
        let expr = Expr::Num(42);
        let expected = "    push 42
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_add_expr() {
        let lhs = Expr::Num(12);
        let rhs = Expr::Num(23);
        let expr = Expr::Add(Box::new(Binary { lhs, rhs }));
        let expected = "    push 12
    push 23
    pop rdi
    pop rax
    add rax, rdi
    push rax
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_nested_add_expr() {
        let lhs = Expr::Add(Box::new(Binary {
            lhs: Expr::Num(12),
            rhs: Expr::Num(23),
        }));
        let rhs = Expr::Num(34);
        let expr = Expr::Add(Box::new(Binary { lhs, rhs }));
        let expected = "    push 12
    push 23
    pop rdi
    pop rax
    add rax, rdi
    push rax
    push 34
    pop rdi
    pop rax
    add rax, rdi
    push rax
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_sub_expr() {
        let lhs = Expr::Num(23);
        let rhs = Expr::Num(12);
        let expr = Expr::Sub(Box::new(Binary { lhs, rhs }));
        let expected = "    push 23
    push 12
    pop rdi
    pop rax
    sub rax, rdi
    push rax
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_mul_expr() {
        let lhs = Expr::Num(2);
        let rhs = Expr::Num(3);
        let expr = Expr::Mul(Box::new(Binary { lhs, rhs }));
        let expected = "    push 2
    push 3
    pop rdi
    pop rax
    imul rax, rdi
    push rax
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn gen_div_expr() {
        let lhs = Expr::Num(4);
        let rhs = Expr::Num(2);
        let expr = Expr::Div(Box::new(Binary { lhs, rhs }));
        let expected = "    push 4
    push 2
    pop rdi
    pop rax
    xor rdx, rdx
    idiv rdi
    push rax
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_gen_function_name() {
        let expr = Expr::FnName("some_func".to_string());
        assert!(gen_expr(&expr).is_err());
    }

    #[test]
    fn gen_function_call() {
        let expr = Expr::FnCall(Box::new(Expr::FnName("some_func".to_string())));
        let expected = "    call some_func
    push rax
";
        let actual = gen_expr(&expr).unwrap();
        assert_eq!(expected, actual);
    }
}
