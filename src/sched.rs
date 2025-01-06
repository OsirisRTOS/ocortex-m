//! Module: sched

/// Type: CtxPtr
pub type CtxPtr = *const u32;

/// Struct: ThreadContext
pub struct ThreadContext {
    ptr: Option<CtxPtr>,
}

impl ThreadContext {
    /// Function: new
    /// Precondition: ptr is a valid pointer to a thread context.
    ///     Especially ptr must satisfy the following conditions:
    ///    - It points to the bottom of the stack of the corresponding thread.
    ///    - The stack must be 4-byte aligned.
    ///    - The layout of the stack must be as follows (from bottom to top):
    ///       r11, r10, r9, r8, r7, r6, r5, r4, r0, r1, r2, r3, r12, lr, pc, xpsr, (if fpu -> s16-31)
    /// Postcondition: A ThreadContext object is returned.
    pub unsafe fn new(ctx: CtxPtr) -> Self {
        ThreadContext { ptr: Some(ctx) }
    }

    /// Create an empty ThreadContext object.
    pub fn empty() -> Self {
        ThreadContext { ptr: None }
    }
}

/*
 * Function: dispatch
 * Precondition: ctx is a valid ThreadContext object.
 * Postcondition: The context is restored and the thread is resumed.
 */
/*pub unsafe fn dispatch(ctx: &ThreadContext) -> ! {
    // The layout of ptr is as follows:
    // ptr[0] = r11, ptr[1] = r10, r9, r8, r7, r6, r5, r4, r0, r1, r2, r3, r12, lr, pc, xpsr
    // We need to and lr with 0x4 to check which stack pointer to use.
    let exec_return = unsafe { ctx.ptr.offset(13).read_volatile() };

    if exec_return & 0x4 == 0 {
        unsafe {
            asm!("msr psp, {}", in(reg) ctx.ptr as u32, options(nostack, preserves_flags));
            asm!("mrs r0, psp", "ldmia r0!, {r4-r11}", options(nomem, preserves_flags))
        };
    } else {
        unsafe {
            asm!("msr msp, {}", in(reg) ctx.ptr as u32, options(nomem, nostack, preserves_flags));
            asm!("mrs r0, msp", "ldmia r0!, {r4-r11}", options(nomem, preserves_flags))
        };
    }

    #[cfg(feature = "has_fpu")]
    if exec_return & 0x10 != 0 {
        unsafe { asm!("vldmia r0!, {s16-s31}", options(nomem, preserves_flags)) };
    }

    unsafe {
        asm!("mov lr, {}", in(reg) exec_return, options(nomem, nostack, preserves_flags));
        asm!("bx lr", options(nomem, nostack, preserves_flags))
    };
}*/
