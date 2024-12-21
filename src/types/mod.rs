//! Services module for the HAL.

/// Thread dispatcher service.
pub mod dispatcher;

/// The return type of the sched_call.
pub type SchedCtx = u32;
