use crate::parser::{Binary, Expr};

pub fn gen(ast: &Expr) -> String {
    let mut asm = "    .intel_syntax noprefix
    .text
    .globl main
main:
"
    .to_string();

    asm.push_str(&gen_expr(ast));
    asm.push_str("    pop rax\n");
    asm.push_str("    ret\n");
    asm
}

fn gen_expr(ast: &Expr) -> String {
    match ast {
        Expr::Num(n) => format!("    push {}\n", n),
        Expr::Add(Binary { lhs, rhs }) => {
            let mut s = gen_expr(lhs);
            s.push_str(&gen_expr(rhs));
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    add rax, rdi\n");
            s.push_str("    push rax\n");
            s
        }
        Expr::Sub(Binary { lhs, rhs }) => {
            let mut s = gen_expr(lhs);
            s.push_str(&gen_expr(rhs));
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    sub rax, rdi\n");
            s.push_str("    push rax\n");
            s
        }
        Expr::Mul(Binary { lhs, rhs }) => {
            let mut s = gen_expr(lhs);
            s.push_str(&gen_expr(rhs));
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    imul rax, rdi\n");
            s.push_str("    push rax\n");
            s
        }
        Expr::Div(Binary { lhs, rhs }) => {
            let mut s = gen_expr(lhs);
            s.push_str(&gen_expr(rhs));
            s.push_str("    pop rdi\n");
            s.push_str("    pop rax\n");
            s.push_str("    xor rdx, rdx\n");
            s.push_str("    idiv rdi\n");
            s.push_str("    push rax\n");
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_single_num_node() {
        let ast = Expr::Num(42);
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

    mod tests_gen_expr {
        use super::*;

        #[test]
        fn gen_add_expr() {
            let lhs = Expr::Num(12);
            let rhs = Expr::Num(23);
            let ast = Expr::Add(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
            let expected = "    push 12
    push 23
    pop rdi
    pop rax
    add rax, rdi
    push rax
";
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }

        #[test]
        fn gen_nested_add_expr() {
            let lhs = Expr::Add(Binary {
                lhs: Box::new(Expr::Num(12)),
                rhs: Box::new(Expr::Num(23)),
            });
            let rhs = Expr::Num(34);
            let ast = Expr::Add(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
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
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }

        #[test]
        fn gen_sub_expr() {
            let lhs = Expr::Num(23);
            let rhs = Expr::Num(12);
            let ast = Expr::Sub(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
            let expected = "    push 23
    push 12
    pop rdi
    pop rax
    sub rax, rdi
    push rax
";
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }

        #[test]
        fn gen_mul_expr() {
            let lhs = Expr::Num(2);
            let rhs = Expr::Num(3);
            let ast = Expr::Mul(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
            let expected = "    push 2
    push 3
    pop rdi
    pop rax
    imul rax, rdi
    push rax
";
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }

        #[test]
        fn gen_div_expr() {
            let lhs = Expr::Num(4);
            let rhs = Expr::Num(2);
            let ast = Expr::Div(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
            let expected = "    push 4
    push 2
    pop rdi
    pop rax
    xor rdx, rdx
    idiv rdi
    push rax
";
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }
    }
}
