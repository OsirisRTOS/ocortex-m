#![allow(missing_docs)]

use super::debug::{self, SEMIH_REPORT_EXCEPTION};

/// Status enum for `exit` syscall.
pub type ExitStatus = Result<(), ()>;

/// Successful execution of a program.
pub const EXIT_SUCCESS: ExitStatus = Ok(());

/// Unsuccessful execution of a program.
pub const EXIT_FAILURE: ExitStatus = Err(());

/// This values are taken from section 5.5.2 of
/// ADS Debug Target Guide (DUI0058).
// TODO document
#[allow(missing_docs)]
pub enum Exception {
    // Hardware reason codes
    BranchThroughZero = 0x20000,
    UndefinedInstr = 0x20001,
    SoftwareInterrupt = 0x20002,
    PrefetchAbort = 0x20003,
    DataAbort = 0x20004,
    AddressException = 0x20005,
    IRQ = 0x20006,
    FIQ = 0x20007,
    // Software reason codes
    BreakPoint = 0x20020,
    WatchPoint = 0x20021,
    StepComplete = 0x20022,
    RunTimeErrorUnknown = 0x20023,
    InternalError = 0x20024,
    UserInterruption = 0x20025,
    ApplicationExit = 0x20026,
    StackOverflow = 0x20027,
    DivisionByZero = 0x20028,
    OSSpecific = 0x20029,
}

/// Reports to the debugger that the execution has completed.
///
/// This call can be used to terminate QEMU session and report back success
/// or failure. If you need to pass more than one type of error, consider
/// using `report_exception` syscall instead.
///
/// This call should not return. However, it is possible for the debugger
/// to request that the application continue. In that case this call
/// returns normally.
///
pub fn exit(status: ExitStatus) {
    match status {
        EXIT_SUCCESS => report_exception(Exception::ApplicationExit),
        EXIT_FAILURE => report_exception(Exception::RunTimeErrorUnknown),
    }
}

/// Report an exception to the debugger directly.
///
/// Exception handlers can use this SWI at the end of handler chains
/// as the default action, to indicate that the exception has not been handled.
///
/// This call should not return. However, it is possible for the debugger
/// to request that the application continue. In that case this call
/// returns normally.
///
/// # Arguments
///
/// * `reason` - A reason code reported back to the debugger.
///
pub fn report_exception(reason: Exception) {
    let code = reason as usize;
    unsafe {
        debug::semih_call_impl(SEMIH_REPORT_EXCEPTION, code);
    }
}
