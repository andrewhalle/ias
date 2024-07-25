#![allow(internal_features)]
#![feature(core_intrinsics)]

use anyhow::Result;
use nix::unistd;

mod assemble;
mod interpret;

fn run_main() -> Result<()> {
    let (to_tracee, from_interpreter) = unistd::pipe()?;
    let tracee = interpret::spawn_traced_thread(from_interpreter)?;
    interpret::run_interpreter_loop(tracee, to_tracee)?;
    anyhow::Ok(())
}

fn main() {
    if let Err(err) = run_main() {
        eprintln!("{err:?}");
    }
}
