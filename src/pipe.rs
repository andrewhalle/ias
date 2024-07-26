use std::{
    io::{self, BufRead as _, BufReader, Read, Write},
    os::fd::{AsRawFd as _, OwnedFd},
};

use nix::unistd;

pub(super) use read::PipeReader;

mod read {
    use super::*;

    #[doc(hidden)]
    pub(super) struct Inner(pub(super) OwnedFd);
    impl Read for Inner {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            Ok(unistd::read(self.0.as_raw_fd(), buf)?)
        }
    }
    pub(in super::super) struct PipeReader(pub(super) BufReader<Inner>);
    impl PipeReader {
        pub(in super::super) fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            self.0.read_line(buf)
        }
    }
}

pub(super) struct PipeWriter(OwnedFd);
impl Write for PipeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(unistd::write(&self.0, buf)?)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(super) fn pipe() -> io::Result<(PipeWriter, PipeReader)> {
    // Below is correct, `unistd::pipe()` returns things in the order of the syscall, which is
    // reversed from the order of Rust's channel interface.
    let (rx, tx) = unistd::pipe()?;
    Ok((PipeWriter(tx), PipeReader(BufReader::new(read::Inner(rx)))))
}
