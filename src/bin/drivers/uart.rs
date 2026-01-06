use core::arch::asm;

const UART_ADDR: usize = 0x6000_0000_usize;

pub fn write_word(word: usize) {
    unsafe {
        asm!("sw {1}, 0({0})", in(reg) UART_ADDR, in(reg) word);
    }
}

pub fn write_buffer(buffer: impl Iterator<Item = usize>) {
    for word in buffer {
        write_word(word);
    }
}

pub fn write_chars(buffer: impl Iterator<Item = char>) {
    for word in buffer {
        write_word(word as usize);
    }
}
