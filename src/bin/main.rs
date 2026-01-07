#![no_std]
#![no_main]

mod kalloc;
mod scheduling;
mod drivers;
use drivers::uart;
use esp_hal::clock::CpuClock;
use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    uart::write_chars(['P', 'a', 'n', 'i', 'c', '\n'].into_iter());
    loop {}
}

#[main]
unsafe fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let init = scheduling::spawn_init();
    uart::write_word((init.id + 65) as usize);
    uart::write_word((init.entry_addr + 66) as usize);
    scheduling::context_switch(&*init);

    loop {}
}
