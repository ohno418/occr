use crate::parser::{Node, Binary};

pub fn gen(ast: &Node) -> String {
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

fn gen_expr(ast: &Node) -> String {
    match ast {
        Node::Num(n) => format!("    push {}\n", n),
        Node::Add(Binary { lhs, rhs }) => {
            let mut s = gen_expr(lhs);
            s.push_str("    pop rdi\n");
            s.push_str(&gen_expr(rhs));
            s.push_str("    pop rax\n");
            s.push_str("    add rax, rdi\n");
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
        let ast = Node::Num(42);
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
            let lhs = Node::Num(12);
            let rhs = Node::Num(23);
            let ast = Node::Add(Binary { lhs: Box::new(lhs), rhs: Box::new(rhs) });
            let expected = "    push 12
    pop rdi
    push 23
    pop rax
    add rax, rdi
    push rax
";
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }

        #[test]
        fn gen_nested_add_expr() {
            let lhs = Node::Add(Binary {
                lhs: Box::new(Node::Num(12)),
                rhs: Box::new(Node::Num(23)),
            });
            let rhs = Node::Num(34);
            let ast = Node::Add(Binary {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
            let expected = "    push 12
    pop rdi
    push 23
    pop rax
    add rax, rdi
    push rax
    pop rdi
    push 34
    pop rax
    add rax, rdi
    push rax
";
            let actual = gen_expr(&ast);
            assert_eq!(expected, actual);
        }
    }
}
