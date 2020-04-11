use core::panic::PanicInfo;
use crate::{log_write};

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    log_write!(
        "\n\n--------------------------------- \
        Kernel Panic \
        ---------------------------------\n"
    );
    if let Some(args) = info.message() {
        log_write!("{}", args)
    }
    loop {}
}

