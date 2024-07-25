#![allow(unused)]

use std::{fs, io, path::Path, process::Command};

/// Assemble a program into machine code.
///
/// This function calls out to an external assembler (currently `as`) and `objcopy` in order to
/// extract the instruction bytes.
pub(super) fn assemble(program: &str) -> io::Result<Vec<u8>> {
    let dir = tempfile::tempdir()?;
    fs::write(dir.as_ref().join("program.s"), program)?;
    call_as(dir.as_ref())?;
    call_objcopy(dir.as_ref())?;
    fs::read(dir.as_ref().join("program.bin"))
}

fn call_as(current_dir: &Path) -> io::Result<()> {
    Command::new("as")
        .current_dir(current_dir)
        .args([
            "-msyntax=intel",
            "-mnaked-reg",
            "-o",
            "program.o",
            "program.s",
        ])
        .output()
        .map(|_| ())
}

fn call_objcopy(current_dir: &Path) -> io::Result<()> {
    Command::new("objcopy")
        .current_dir(current_dir)
        .args([
            "-O",
            "binary",
            "--only-section=.text",
            "program.o",
            "program.bin",
        ])
        .output()
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_program() {
        assert_eq!(
            assemble("mov eax, 0\nret").unwrap(),
            b"\xb8\x00\x00\x00\x00\xc3",
        );
    }
}
