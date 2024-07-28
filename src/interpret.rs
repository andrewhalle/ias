use std::io::{self, Write as _};

use anyhow::Result;
use nix::{
    sys::{
        ptrace::{self, regset::NT_PRSTATUS},
        wait,
    },
    unistd::{self, ForkResult, Pid},
};

use crate::pipe::{PipeReader, PipeWriter};

pub(super) fn run_interpreter_loop(tracee: Pid, mut pipe_to_tracee: PipeWriter) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();
    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        if !line.ends_with('\n') {
            line.push('\n');
            println!();
        }
        pipe_to_tracee.write_all(line.as_bytes())?;
        ptrace::cont(tracee, None)?;
        let wait_status = wait::waitpid(tracee, None)?;
        stdout.flush()?;
        println!("child stopped: wait status {wait_status:?}");
        dump_registers(tracee)?;
        line = String::new();
    }
}

fn dump_registers(tracee: Pid) -> Result<()> {
    let registers = ptrace::getregset::<NT_PRSTATUS>(tracee)?;
    println!("registers: {registers:#?}");
    Ok(())
}

// Once we're in the child, we can unwrap freely, our parent will be notified if we panic.
fn run_child(mut lines: PipeReader) {
    let mut stdout = io::stdout();
    ptrace::traceme().unwrap();
    let mut line = String::new();
    loop {
        unsafe {
            std::intrinsics::breakpoint();
        }
        lines.read_line(&mut line).unwrap();
        println!("child: got line: {}", line.trim());
        println!("child: <assemble instructions here>");
        println!("child: <jump to code>");
        stdout.flush().unwrap();
        line = String::new();
    }
}

pub(super) fn spawn_traced_thread(lines: PipeReader) -> Result<Pid> {
    let fork_result = unsafe { unistd::fork() }?;
    match fork_result {
        ForkResult::Parent { child } => Ok(child),
        ForkResult::Child => {
            run_child(lines);
            unreachable!()
        }
    }
}
