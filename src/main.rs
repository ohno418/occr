use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Required only one argument.");
        process::exit(1)
    }

    let asm = match compile(&args[1]) {
        Ok(asm) => asm,
        Err(err) => {
            eprintln!("failed to compiler: {}", err);
            process::exit(1);
        }
    };
    print!("{}", asm);
}

pub fn compile(input: &str) -> Result<String, String> {
    let num: i32 = match input.parse() {
        Ok(n) => n,
        Err(_) => return Err("No".to_string()),
    };

    Ok(format!(
        "\
    .intel_syntax noprefix
    .text
    .globl main
main:
    mov rax, {}
    ret
",
        num
    ))
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
