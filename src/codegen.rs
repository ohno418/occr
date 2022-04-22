use crate::parser::Node;

pub fn gen(ast: &Node) -> String {
    let mut asm = "\
    .intel_syntax noprefix
    .text
    .globl main
main:
"
    .to_string();

    match ast {
        Node::Num(n) => asm.push_str(format!("    mov rax, {}\n", n).as_str()),
    };

    asm.push_str("    ret\n");
    asm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_single_num_node() {
        let ast = Node::Num(42);
        let expected = "\
    .intel_syntax noprefix
    .text
    .globl main
main:
    mov rax, 42
    ret
";
        let actual = gen(&ast);

        assert_eq!(expected, actual);
    }
}
