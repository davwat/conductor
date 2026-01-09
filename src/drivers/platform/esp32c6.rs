use crate::scheduling;
use crate::drivers::uart;
use esp_hal::main;
use esp_hal::clock::CpuClock;

pub const UART_ADDR: usize = 0x6000_0000_usize;

esp_bootloader_esp_idf::esp_app_desc!(); // TODO: possibly re-implement this macro

pub fn platform_init() {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
}

#[main]
unsafe fn main() -> ! {
    let init = scheduling::spawn_init();
    uart::write_word((init.id + 65) as usize);
    uart::write_word((init.entry_addr + 65) as usize);
    scheduling::context_switch(&*init);

    loop {}
}
