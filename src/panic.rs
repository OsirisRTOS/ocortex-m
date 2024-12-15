#![deny(warnings)]
#![allow(missing_docs)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(all(feature = "panic-semih", target_arch = "arm"))]
    {
        use crate::interrupt;
        use crate::semih::hio;
        use core::fmt::Write;

        interrupt::disable();

        if let Ok(mut hstdout) = hio::hstdout() {
            writeln!(hstdout, "{}", _info).ok();
        }
    }

    #[cfg(all(not(test), not(doctest)))]
    #[cfg(all(feature = "panic-itm", target_arch = "arm"))]
    {
        use crate::interrupt;
        use crate::iprintln;
        use crate::peripheral::ITM;
        use core::fmt::Write;

        interrupt::disable();

        let itm = unsafe { &mut *ITM::PTR };
        let stim = &mut itm.stim[0];

        iprintln!(stim, "{}", _info);
    }

    match () {
        #[cfg(all(feature = "panic-exit"))]
        () => {
            use crate::semih::exit;
            exit::exit(exit::EXIT_FAILURE);
            unreachable!()
        }
        #[cfg(not(feature = "panic-exit"))]
        () => {
            use core::sync::atomic::{self, Ordering};

            #[cfg(feature = "panic-semih")]
            {
                use crate::asm;
                asm::bkpt();
            }

            loop {
                // add some side effect to prevent this from turning into a UDF instruction
                // see rust-lang/rust#28728 for details
                atomic::compiler_fence(Ordering::SeqCst);
            }
        }
    }
}
