fn main() {
    let asm = compile();
    print!("{}", asm);
}

pub fn compile() -> String {
    "\
    .intel_syntax noprefix
    .text
    .globl main
main:
    mov rax, 42
    ret
".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_asm() {
        let expected = "\
    .intel_syntax noprefix
    .text
    .globl main
main:
    mov rax, 42
    ret
";
        let actual = compile();
        assert_eq!(expected, actual);
    }
}
