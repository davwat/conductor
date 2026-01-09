use crate::drivers::uart;

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    uart::write_chars(['P', 'a', 'n', 'i', 'c', '\n'].into_iter());
    loop {}
}
