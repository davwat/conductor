#![no_std]
#![no_main]

use core::arch::asm;
use esp_hal::clock::CpuClock;
use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn uart_write_word(word: usize) {
    let uart_address = 0x6000_0000_usize;
    unsafe {
        asm!("sw {1}, 0({0})", in(reg) uart_address, in(reg) word);
    }
}

#[main]
unsafe fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let trap_handle_addr: usize = trap_handle as usize;
    asm!("csrw mepc, {0}", in(reg) trap_handle_addr);
    asm!("mret");
    uart_write_word(69);
    uart_write_word(76);
    uart_write_word(76);
    uart_write_word(79);

    let test = 1234;
    let test_ptr: *const i32 = &test;
    let mut test_addr: usize = test_ptr as usize;
    loop {
        uart_write_word(test_addr % 10 + 48);
        test_addr /= 10;
        if test_addr == 0 { break; }
    }

    loop {}
}

unsafe fn trap_handle() -> ! {
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    loop {}
}
