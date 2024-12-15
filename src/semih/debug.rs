#![allow(missing_docs)]

pub const SEMIH_CLOCK: usize = 0x10;
pub const SEMIH_CLOSE: usize = 0x02;
pub const SEMIH_ELAPSED: usize = 0x30;
pub const SEMIH_ERRNO: usize = 0x13;
pub const SEMIH_FLEN: usize = 0x0c;
pub const SEMIH_GET_CMDLINE: usize = 0x15;
pub const SEMIH_HEAPINFO: usize = 0x16;
pub const SEMIH_ISERROR: usize = 0x08;
pub const SEMIH_ISTTY: usize = 0x09;
pub const SEMIH_OPEN: usize = 0x01;
pub const SEMIH_READ: usize = 0x06;
pub const SEMIH_READC: usize = 0x07;
pub const SEMIH_REMOVE: usize = 0x0e;
pub const SEMIH_RENAME: usize = 0x0f;
pub const SEMIH_SEEK: usize = 0x0a;
pub const SEMIH_SYSTEM: usize = 0x12;
pub const SEMIH_TICKFREQ: usize = 0x31;
pub const SEMIH_TIME: usize = 0x11;
pub const SEMIH_TMPNAM: usize = 0x0d;
pub const SEMIH_WRITE0: usize = 0x04;
pub const SEMIH_WRITE: usize = 0x05;
pub const SEMIH_WRITEC: usize = 0x03;
pub const SEMIH_ENTER_SVC: usize = 0x17;
pub const SEMIH_REPORT_EXCEPTION: usize = 0x18;

#[macro_export]
macro_rules! pack_args {
    ($($val:expr),*) => {
        [$($val as usize),*]
    };
}

/// Performs a semihosting operation, takes a pointer to an argument block
///
/// # Safety
///
/// The syscall number must be a valid [semihosting operation],
/// and the arguments must be valid for the associated operation.
///
/// [semihosting operation]: https://developer.arm.com/documentation/dui0471/i/semihosting/semihosting-operations?lang=en
#[inline(always)]
pub unsafe fn semih_call<T: ?Sized>(nr: usize, arg: &T) -> usize {
    semih_call_impl(nr, arg as *const T as *const () as usize)
}

/// Performs a semihosting operation, takes one integer as an argument
///
/// # Safety
///
/// Same as [`semih_call`].
#[inline(always)]
pub unsafe fn semih_call_impl(_nr: usize, _arg: usize) -> usize {
    match () {
        #[cfg(all(thumb, feature = "semih"))]
        () => {
            use core::arch::asm;
            let mut nr = _nr as u32;
            let arg = _arg as u32;
            asm!("bkpt #0xab", inout("r0") nr, in("r1") arg, options(nostack, preserves_flags));

            match nr as usize {
                SEMIH_WRITE0 => 0,
                SEMIH_WRITEC => 0,
                x => x,
            }
        }
        #[cfg(all(thumb, not(feature = "semih")))]
        () => 0,
        #[cfg(not(thumb))]
        () => 0,
    }
}
