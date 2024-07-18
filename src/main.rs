use std::io::{self, Read as _, Write as _};

mod assemble;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut program = String::new();
    stdin.read_to_string(&mut program).unwrap();
    stdout
        .write_all(&assemble::assemble(&program).unwrap())
        .unwrap();
}
