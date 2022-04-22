mod codegen;
mod lexer;
mod parser;

pub fn compile(input: &str) -> Result<String, String> {
    let tokens = lexer::tokenize(input)?;
    let ast = parser::parse(&tokens)?;
    let asm = codegen::gen(&ast);
    Ok(asm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_asm_to_return_input_number() {
        let expected = "\
    .intel_syntax noprefix
    .text
    .globl main
main:
    mov rax, 42
    ret
";
        let actual = compile("42").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn returns_err_with_not_number_input() {
        assert!(compile("hi").is_err());
    }
}
