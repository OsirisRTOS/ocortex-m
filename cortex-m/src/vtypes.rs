//!------------------------------------------------------------------------------
//! This is a copy of the vcell crate at: https://github.com/japaric/vcell
//! as well as the volatile-register crate at: https://github.com/rust-embedded/volatile-register
//! We decided to copy the code instead of adding a dependency to the crate because it is a small crate and we want to avoid adding dependencies to the project.
//!------------------------------------------------------------------------------

//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html

//! Volatile access to memory mapped hardware registers
//!
//! # Usage
//!
//! ``` no_run
//! use volatile_register::RW;
//!
//! // Create a struct that represents the memory mapped register block
//! /// Nested Vector Interrupt Controller
//! #[repr(C)]
//! pub struct Nvic {
//!     /// Interrupt Set-Enable
//!     pub iser: [RW<u32>; 8],
//!     reserved0: [u32; 24],
//!     /// Interrupt Clear-Enable
//!     pub icer: [RW<u32>; 8],
//!     reserved1: [u32; 24],
//!     // .. more registers ..
//! }
//!
//! // Access the registers by casting the base address of the register block
//! // to the previously declared `struct`
//! let nvic = 0xE000_E100 as *const Nvic;
//! // Unsafe because the compiler can't verify the address is correct
//! unsafe { (*nvic).iser[0].write(1) }
//! ```

/*Copyright (c) 2017 Jorge Aparicio

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.*/

#![deny(missing_docs)]
#![deny(warnings)]

use core::cell::UnsafeCell;
use core::ptr;

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        VolatileCell {
            value: UnsafeCell::new(value),
        }
    }

    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { ptr::read_volatile(self.value.get()) }
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
    where
        T: Copy,
    {
        unsafe { ptr::write_volatile(self.value.get(), value) }
    }

    /// Returns a raw pointer to the underlying data in the cell
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}

//------------------------------------------------------------------------------

/*Copyright (c) 2016 Jorge Aparicio

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.*/

/// Read-Only register
#[repr(transparent)]
pub struct RO<T>
where
    T: Copy,
{
    register: VolatileCell<T>,
}

impl<T> RO<T>
where
    T: Copy,
{
    /// Reads the value of the register
    #[inline(always)]
    pub fn read(&self) -> T {
        self.register.get()
    }
}

/// Read-Write register
#[repr(transparent)]
pub struct RW<T>
where
    T: Copy,
{
    register: VolatileCell<T>,
}

impl<T> RW<T>
where
    T: Copy,
{
    /// Performs a read-modify-write operation
    ///
    /// NOTE: `unsafe` because writes to a register are side effectful
    #[inline(always)]
    pub unsafe fn modify<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.register.set(f(self.register.get()));
    }

    /// Reads the value of the register
    #[inline(always)]
    pub fn read(&self) -> T {
        self.register.get()
    }

    /// Writes a `value` into the register
    ///
    /// NOTE: `unsafe` because writes to a register are side effectful
    #[inline(always)]
    pub unsafe fn write(&self, value: T) {
        self.register.set(value)
    }
}

/// Write-Only register
#[repr(transparent)]
pub struct WO<T>
where
    T: Copy,
{
    register: VolatileCell<T>,
}

impl<T> WO<T>
where
    T: Copy,
{
    /// Writes `value` into the register
    ///
    /// NOTE: `unsafe` because writes to a register are side effectful
    #[inline(always)]
    pub unsafe fn write(&self, value: T) {
        self.register.set(value)
    }
}
