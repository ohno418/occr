use occr::compile;

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
