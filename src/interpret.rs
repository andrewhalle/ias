use std::{
    io::{self, Write as _},
    os::fd::OwnedFd,
};

use anyhow::Result;
use nix::{
    sys::{
        ptrace::{self, regset::NT_PRSTATUS},
        wait,
    },
    unistd::{self, ForkResult, Pid},
};

pub(super) fn run_interpreter_loop(tracee: Pid, _pipe_to_tracee: OwnedFd) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();
    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut line)?;
        if !line.ends_with('\n') {
            println!();
        }
        // pipe_to_tracee.send(line)?;
        println!("<send line to tracee>");
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
fn run_child(_lines: OwnedFd) {
    let mut stdout = io::stdout();
    ptrace::traceme().unwrap();
    loop {
        unsafe {
            std::intrinsics::breakpoint();
        }
        println!("child: <get line from shell>");
        println!("child: <assemble instructions here>");
        println!("child: <jump to code>");
        stdout.flush().unwrap();
    }
}

pub(super) fn spawn_traced_thread(lines: OwnedFd) -> Result<Pid> {
    let fork_result = unsafe { unistd::fork() }?;
    match fork_result {
        ForkResult::Parent { child } => Ok(child),
        ForkResult::Child => {
            run_child(lines);
            unreachable!()
        }
    }
}
