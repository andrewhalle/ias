#![allow(internal_features)]
#![feature(core_intrinsics)]

use anyhow::Result;

mod assemble;
mod interpret;
mod pipe;

fn run_main() -> Result<()> {
    let (to_tracee, from_interpreter) = pipe::pipe()?;
    let tracee = interpret::spawn_traced_thread(from_interpreter)?;
    interpret::run_interpreter_loop(tracee, to_tracee)?;
    anyhow::Ok(())
}

fn main() {
    if let Err(err) = run_main() {
        eprintln!("{err:?}");
    }
}
