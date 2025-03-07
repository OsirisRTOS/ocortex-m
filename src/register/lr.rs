//! Link register

#[cfg(cortex_m)]
use core::arch::asm;

/// Reads the CPU register
#[cfg(cortex_m)]
#[inline]
pub fn read() -> u32 {
    let r;
    unsafe { asm!("mov {}, lr", out(reg) r, options(nomem, nostack, preserves_flags)) };
    r
}

/// Writes `bits` to the CPU register
#[cfg(cortex_m)]
#[inline]
pub unsafe fn write(bits: u32) {
    asm!("mov lr, {}", in(reg) bits, options(nomem, nostack, preserves_flags));
}

///
#[macro_export]
macro_rules! irq_is_msp {
    () => {
        {
            let lr: u32;
            unsafe { asm!("mov {}, lr", out(reg) lr, options(nostack, preserves_flags)) };
            lr & 0x4 == 0
        }
    };
}

pub use irq_is_msp;
