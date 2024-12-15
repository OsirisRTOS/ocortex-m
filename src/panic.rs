#![deny(warnings)]
#![allow(missing_docs)]

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(all(feature = "panic-semih", target_arch = "arm"))]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(not(feature = "exit"))]
    use crate::asm;
    #[cfg(feature = "exit")]
    use crate::debug::{self, EXIT_FAILURE};
    use crate::interrupt;
    use crate::semih::hio;
    use core::fmt::Write;

    interrupt::disable();

    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{}", info).ok();
    }

    match () {
        // Exit the QEMU process
        #[cfg(feature = "exit")]
        () => debug::exit(EXIT_FAILURE),
        // OK to fire a breakpoint here because we know the microcontroller is connected to a
        // debugger
        #[cfg(not(feature = "exit"))]
        () => asm::bkpt(),
    }

    loop {}
}

#[panic_handler]
#[cfg(all(not(test), not(doctest)))]
#[cfg(all(feature = "panic-itm", target_arch = "arm"))]
fn panic(info: &PanicInfo) -> ! {
    use crate::interrupt;
    use crate::iprintln;
    use crate::peripheral::ITM;
    use core::fmt::Write;
    use core::sync::atomic::{self, Ordering};

    interrupt::disable();

    let itm = unsafe { &mut *ITM::PTR };
    let stim = &mut itm.stim[0];

    iprintln!(stim, "{}", info);

    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[panic_handler]
#[cfg(all(not(feature = "panic-semih"), not(feature = "panic-itm")))]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
