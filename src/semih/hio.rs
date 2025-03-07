//! Host I/O

// Fixing this lint requires a breaking change that does not add much value
#![allow(clippy::result_unit_err)]

use core::{ffi::CStr, fmt, slice};

use crate::{pack_args, sync::OnceCell};

use super::{
    debug::{semih_call, SEMIH_OPEN, SEMIH_WRITE, SEMIH_WRITE0},
    open::{W_APPEND, W_TRUNC},
};

/// The host's standard output stream.
static HSTDOUT: OnceCell<Result<HostStream, Error>> = OnceCell::new();
/// The host's standard error stream.
static HSTDERR: OnceCell<Result<HostStream, Error>> = OnceCell::new();

#[derive(Debug, Clone, Copy)]
pub enum Error {
    OpenFailed,
    FmtFailed(fmt::Error),
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::FmtFailed(e)
    }
}

/// A byte stream to the host (e.g., host's stdout or stderr).
#[derive(Clone, Copy)]
pub struct HostStream {
    fd: usize,
}

impl HostStream {
    /// Attempts to write an entire `buffer` into this sink
    pub fn write_all(&self, buffer: &[u8]) -> Result<(), ()> {
        write_all(self.fd, buffer)
    }
}

impl fmt::Write for HostStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// Construct a new handle to the host's standard error.
pub fn hstderr() -> &'static Result<HostStream, Error> {
    // There is actually no stderr access in ARM Semihosting documentation. Use
    // convention used in libgloss.
    // See: libgloss/arm/syscalls.c, line 139.
    // https://sourceware.org/git/gitweb.cgi?p=newlib-cygwin.git;a=blob;f=libgloss/arm/syscalls.c#l139
    HSTDERR.do_or_get(|| open(":tt\0", W_APPEND))
}

/// Construct a new handle to the host's standard output.
pub fn hstdout() -> &'static Result<HostStream, Error> {
    HSTDOUT.do_or_get(|| open(":tt\0", W_TRUNC))
}

fn open(name: &str, mode: usize) -> Result<HostStream, Error> {
    let name = name.as_bytes();
    match unsafe { semih_call(SEMIH_OPEN, &pack_args!(name.as_ptr(), mode, name.len() - 1)) }
        as isize
    {
        -1 => Err(Error::OpenFailed),
        fd => Ok(HostStream { fd: fd as usize }),
    }
}

fn write_all(fd: usize, mut buffer: &[u8]) -> Result<(), ()> {
    while !buffer.is_empty() {
        match unsafe { semih_call(SEMIH_WRITE, &pack_args!(fd, buffer.as_ptr(), buffer.len())) } {
            // Done
            0 => return Ok(()),
            // `n` bytes were not written
            n if n <= buffer.len() => {
                let offset = (buffer.len() - n) as isize;
                buffer = unsafe { slice::from_raw_parts(buffer.as_ptr().offset(offset), n) }
            }
            #[cfg(feature = "jlink-quirks")]
            // Error (-1) - should be an error but JLink can return -1, -2, -3,...
            // For good measure, we allow up to negative 15.
            n if n > 0xfffffff0 => return Ok(()),
            // Error
            _ => return Err(()),
        }
    }

    Ok(())
}

/// Writes a C string to the host's debug console.
pub fn write_debug(buffer: &CStr) {
    unsafe { semih_call(SEMIH_WRITE0, buffer.to_bytes_with_nul()) };
}
