/// Macro for sending a formatted string through an ITM channel
#[macro_export]
macro_rules! iprint {
    ($channel:expr, $s:expr) => {
        $crate::itm::write_str($channel, $s);
    };
    ($channel:expr, $($arg:tt)*) => {
        $crate::itm::write_fmt($channel, format_args!($($arg)*));
    };
}

/// Macro for sending a formatted string through an ITM channel, with a newline.
#[macro_export]
macro_rules! iprintln {
    ($channel:expr) => {
        $crate::itm::write_str($channel, "\n");
    };
    ($channel:expr, $fmt:expr) => {
        $crate::itm::write_str($channel, concat!($fmt, "\n"));
    };
    ($channel:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::itm::write_fmt($channel, format_args!(concat!($fmt, "\n"), $($arg)*));
    };
}

/// Macro for sending a formatted string to the host's standard output, using semi-hosting, with a newline.
#[macro_export]
macro_rules! hprintln {
    () => {
        {
            use core::fmt::Write;
            $crate::semih::hio::hstdout().map(|mut hstdout| hstdout.write_str("\n"))
        }

    };
    ($fmt:expr) => {
        {
            use core::fmt::Write;

            match $crate::semih::hio::hstdout()
            .map(|mut hstdout| hstdout.write_fmt(format_args!(concat!($fmt, "\n"))))
            {
                Ok(result) => result.map_err(|x| x.into()),
                Err(err) => Err(err),
            }
        }

    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            use core::fmt::Write;

            match $crate::semih::hio::hstdout()
            .map(|mut hstdout| hstdout.write_fmt(format_args!(concat!($fmt, "\n"), $($arg)*)))
            {
                Ok(result) => result.map_err(|x| x.into()),
                Err(err) => Err(err),
            }
        }
    };
}

/// Macro for doing a system call.
#[macro_export]
macro_rules! syscall {
    ($num:expr) => {
        unsafe {
            asm!("svc {0}", $num);
        }
    };
    ($num:expr, $arg0:expr) => {
        unsafe {
            asm!("mov r0, {0}", "svc {1}", in(reg)$arg0, const $num);
        }
    };
    ($num:expr, $arg0:expr, arg1:expr) => {
        unsafe {
            asm!("mov r0, {0}", "mov r1, {1}", "svc {2}", in(reg)$arg0, in(reg)$arg1, const $num);
        }
    };
    ($num:expr, $arg0:expr, arg1:expr, arg2:expr) => {
        unsafe {
            asm!("mov r0, {0}", "mov r1, {1}", "mov r2, {2}", "svc {3}", in(reg)$arg0, in(reg)$arg1, in(reg)$arg2, const $num);
        }
    };
    ($num:expr, $arg0:expr, arg1:expr, arg2:expr, arg3:expr) => {
        unsafe {
            asm!("mov r0, {0}", "mov r1, {1}", "mov r2, {2}", "mov r3, {3}", "svc {4}", in(reg)$arg0, in(reg)$arg1, in(reg)$arg2, in(reg)$arg3, const $num);
        }
    };
}

/// Macro to create a mutable reference to a statically allocated value
///
/// This macro returns a value with type `Option<&'static mut $ty>`. `Some($expr)` will be returned
/// the first time the macro is executed; further calls will return `None`. To avoid `unwrap`ping a
/// `None` variant the caller must ensure that the macro is called from a function that's executed
/// at most once in the whole lifetime of the program.
///
/// # Notes
///
/// This macro requires a `critical-section` implementation to be set. For most single core systems,
/// you can enable the `critical-section-single-core` feature for this crate. For other systems, you
/// have to provide one from elsewhere, typically your chip's HAL crate.
///
/// For debuggability, you can set an explicit name for a singleton.  This name only shows up the
/// the debugger and is not referencable from other code.  See example below.
///
/// # Example
///
/// ``` no_run
/// use cortex_m::singleton;
///
/// fn main() {
///     // OK if `main` is executed only once
///     let x: &'static mut bool = singleton!(: bool = false).unwrap();
///
///     let y = alias();
///     // BAD this second call to `alias` will definitively `panic!`
///     let y_alias = alias();
/// }
///
/// fn alias() -> &'static mut bool {
///     singleton!(: bool = false).unwrap()
/// }
///
/// fn singleton_with_name() {
///     // A name only for debugging purposes
///     singleton!(FOO_BUFFER: [u8; 1024] = [0u8; 1024]);
/// }
/// ```
#[macro_export]
macro_rules! singleton {
    ($(#[$meta:meta])* $name:ident: $ty:ty = $expr:expr) => {
        $crate::_export::critical_section::with(|_| {
            // this is a tuple of a MaybeUninit and a bool because using an Option here is
            // problematic:  Due to niche-optimization, an Option could end up producing a non-zero
            // initializer value which would move the entire static from `.bss` into `.data`...
            $(#[$meta])*
            static mut $name: (::core::mem::MaybeUninit<$ty>, bool) =
                (::core::mem::MaybeUninit::uninit(), false);

            #[allow(unsafe_code)]
            let used = unsafe { $name.1 };
            if used {
                None
            } else {
                let expr = $expr;

                #[allow(unsafe_code)]
                unsafe {
                    $name.1 = true;
                    Some($name.0.write(expr))
                }
            }
        })
    };
    ($(#[$meta:meta])* : $ty:ty = $expr:expr) => {
        $crate::singleton!($(#[$meta])* VAR: $ty = $expr)
    };
}

/// ``` compile_fail
/// use cortex_m::singleton;
///
/// fn foo() {
///     // check that the call to `uninitialized` requires unsafe
///     singleton!(: u8 = std::mem::uninitialized());
/// }
/// ```
#[allow(dead_code)]
const CFAIL: () = ();

/// ```
/// #![deny(unsafe_code)]
/// use cortex_m::singleton;
///
/// fn foo() {
///     // check that calls to `singleton!` don't trip the `unsafe_code` lint
///     singleton!(: u8 = 0);
/// }
/// ```
#[allow(dead_code)]
const CPASS: () = ();

/// ```
/// use cortex_m::singleton;
///
/// fn foo() {
///     // check that attributes are forwarded
///     singleton!(#[link_section = ".bss"] FOO: u8 = 0);
///     singleton!(#[link_section = ".bss"]: u8 = 1);
/// }
/// ```
#[allow(dead_code)]
const CPASS_ATTR: () = ();
