//! Module: sched

use crate::peripheral;

/// Type: CtxPtr
pub type CtxPtr = *const u32;

/// Struct: ThreadDesc
pub struct ThreadDesc {
    /// The number of arguments passed to the thread.
    pub argc: usize,

    /// The arguments passed to the thread.
    pub argv: *const u8,

    /// The finalizer function to call when the thread is done.
    pub finalizer: extern "C" fn(),

    /// The entry point of the thread.
    pub entry: extern "C" fn(argc: usize, argv: *const *const u8),
}

/// Struct: ThreadContext
#[derive(Debug, Clone, Copy)]
pub struct ThreadContext {
    ptr: CtxPtr,
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
        ThreadContext { ptr: ctx }
    }

    /// Function: from_empty
    /// Precondition: stack is a valid pointer to the top of the stack of the thread.
    ///    Especially stack must satisfy the following conditions:
    ///   - The stack must be 4-byte aligned.
    ///   - The stack must be empty.
    ///   - The stack must be large enough to hold all the registers.
    /// Postcondition: A ThreadContext object is returned.
    ///   The stack is initialized with the default values for the registers.
    ///   The stack pointer can be safely used as a return value for an exception handler.
    pub unsafe fn from_empty(stack: *mut u8, desc: ThreadDesc) -> Self {
        // The stack has to contain all the caller-saved registers.
        // The layout is as follows:
        // xPSR
        // PC (entry point)
        // LR (function to return after the thread is done)
        // R12 (scratch register)
        // R3 (argument to the function - 0)
        // R2 (argument to the function - 0)
        // R1 (argument to the function - argv)
        // R0 (argument to the function - argc)
        // LR (EXEC_RETURN)
        // R11 - R4 (scratch - 0)

        let stack = stack as *mut usize;
        let mut stack = stack.byte_sub(size_of::<u32>() * 8);

        // Set the xPSR register to the default value. (Only the thumb-state bit is set)
        *stack = 1 << 24;

        // Set the PC register to the entry point of the thread.
        stack = stack.sub(1);
        *stack = desc.entry as usize;

        // Set the LR register to the function to return to after the thread is done.
        stack = stack.sub(1);
        *stack = desc.finalizer as usize;

        // Set the R12 register to a scratch register.
        stack = stack.sub(1);
        *stack = 0;

        // Set the R3 register to 0.
        stack = stack.sub(1);
        *stack = 0;

        // Set the R1 register to 0.
        stack = stack.sub(1);
        *stack = 0;

        // Set the R1 register to argv.
        stack = stack.sub(1);
        *stack = desc.argv as usize;

        // Set the R0 register to argc.
        stack = stack.sub(1);
        *stack = desc.argc;

        // Set the LR register to return to thread and PSP.
        stack = stack.sub(1);
        *stack = 0xFFFFFFFD;

        // Set the remaining registers to 0.
        for _ in 0..8 {
            stack = stack.sub(1);
            *stack = 0;
        }

        Self {
            ptr: stack as CtxPtr,
        }
    }
}

impl From<CtxPtr> for ThreadContext {
    fn from(ctx: CtxPtr) -> Self {
        unsafe { ThreadContext::new(ctx) }
    }
}

impl From<ThreadContext> for CtxPtr {
    fn from(ctx: ThreadContext) -> Self {
        ctx.ptr
    }
}

/// Reschedule the tasks.
pub fn reschedule() {
    // Call PendSV to reschedule the tasks.
    peripheral::SCB::set_pendsv();
}
