#![no_std]
#![no_main]

use core::arch::asm;
use esp_hal::clock::CpuClock;
use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    uart_write_word(90);
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

    // set mstatus so that mret 'returns' to user mode - bits 12-11 determine the privilege level
    // to return to (0b00 for U which we want, 0b01 for S but we don't care about that, 0b11 for M)
    // TODO: that

    // set the trap vector to point to syscall_handle()
    let syscall_handle_addr = syscall_handle as usize;
    asm!("csrw mtvec, {0}", in(reg) syscall_handle_addr);

    // set the return address (ie the 'entry point' for the user-mode code) to be trap_handle
    let trap_handle_addr = trap_handle as usize;
    asm!("csrw mepc, {0}", in(reg) trap_handle_addr);
    // switch to user mode and jump to trap_handle()
    asm!("mret");
    uart_write_word(69);
    uart_write_word(76);
    uart_write_word(76);
    uart_write_word(79);

    loop {}
}

#[no_mangle]
extern "C" fn syscall_handle() {
    uart_write_word(65);
    loop {}
}

#[no_mangle]
unsafe extern "C" fn trap_handle() -> ! {
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    uart_write_word(80);
    //asm!("li a7, 0");
    asm!("ecall");
    uart_write_word(81);
    loop {}
}
