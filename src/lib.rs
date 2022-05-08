mod codegen;
mod lexer;
mod parser;
mod ty;

pub fn compile(input: &str) -> Result<String, String> {
    let tokens = lexer::tokenize(input)?;
    let ast = parser::parse(&tokens)?;
    let asm = codegen::gen(&ast)?;
    Ok(asm)
}
