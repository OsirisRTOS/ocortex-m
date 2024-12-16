//! Atomic abstractions for single and multi-core systems.

#[cfg(all(
    feature = "multi-core",
    not(target_has_atomic = "8"),
    not(feature = "atomic-cas")
))]
compile_error!(
    "The `multi-core` feature requires atomic-cas operations to be available on the target. Enable the `atomic-cas` feature."
);

// ----------------------------AtomicU8----------------------------

pub use core::sync::atomic::Ordering;

#[cfg(all(not(feature = "atomic-cas"), cortex_m))]
use core::cell::UnsafeCell;

#[cfg(all(not(feature = "atomic-cas"), cortex_m))]
/// An atomic `u8`.
pub struct AtomicU8 {
    value: UnsafeCell<u8>,
}

#[cfg(not(all(not(feature = "atomic-cas"), cortex_m)))]
pub use core::sync::atomic::AtomicU8;

#[cfg(all(not(feature = "atomic-cas"), cortex_m))]
impl AtomicU8 {
    /// Creates a new atomic u8.
    pub const fn new(value: u8) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Loads the value.
    pub fn load(&self, _: Ordering) -> u8 {
        use crate::interrupt;
        interrupt::free(|| unsafe { *self.value.get() })
    }

    /// Stores a value.
    pub fn store(&self, value: u8, _: Ordering) {
        use crate::interrupt;
        interrupt::free(|| unsafe {
            *self.value.get() = value;
        });
    }

    /// Compares the value and exchanges it.
    pub fn compare_exchange(
        &self,
        current: u8,
        new: u8,
        _: Ordering,
        _: Ordering,
    ) -> Result<u8, u8> {
        use crate::interrupt;
        interrupt::free(|| {
            // Safety:
            // 1. This is safe because we are on a single-core system, in an interrupt-free context.
            // 2. No reference to the value can be acquired outside of this type.
            let value: u8 = unsafe { *self.value.get() };

            if value == current {
                // Safety:
                // 1. This is safe because we are on a single-core system, in an interrupt-free context.
                // 2. No reference to the value can be acquired outside of this type.
                // 3. The local `value` has been copied and not referenced.
                unsafe {
                    *self.value.get() = new;
                }
                Ok(value)
            } else {
                Err(value)
            }
        })
    }
}

#[cfg(not(all(not(feature = "atomic-cas"), cortex_m)))]
pub use core::sync::atomic::AtomicBool;

#[cfg(all(not(feature = "atomic-cas"), cortex_m))]
/// An atomic `bool`.
pub struct AtomicBool {
    value: UnsafeCell<bool>,
}

#[cfg(all(not(feature = "atomic-cas"), cortex_m))]
impl AtomicBool {
    /// Creates a new atomic bool.
    pub const fn new(value: bool) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Loads the value.
    pub fn load(&self, _: Ordering) -> bool {
        use crate::interrupt;
        interrupt::free(|| unsafe { *self.value.get() })
    }

    /// Stores a value.
    pub fn store(&self, value: bool, _: Ordering) {
        use crate::interrupt;
        interrupt::free(|| unsafe {
            *self.value.get() = value;
        });
    }

    /// Compares the value and exchanges it.
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        _: Ordering,
        _: Ordering,
    ) -> Result<bool, bool> {
        use crate::interrupt;
        interrupt::free(|| {
            // Safety:
            // 1. This is safe because we are on a single-core system, in an interrupt-free context.
            // 2. No reference to the value can be acquired outside of this type.
            let value: bool = unsafe { *self.value.get() };

            if value == current {
                // Safety:
                // 1. This is safe because we are on a single-core system, in an interrupt-free context.
                // 2. No reference to the value can be acquired outside of this type.
                // 3. The local `value` has been copied and not referenced.
                unsafe {
                    *self.value.get() = new;
                }
                Ok(value)
            } else {
                Err(value)
            }
        })
    }
}
