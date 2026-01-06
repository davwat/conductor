use core::arch::asm;
extern crate alloc;
use alloc::boxed::Box;
use crate::drivers::uart;

// conductor's 'process control block'
// processes should live on the heap and exactly one instance of Process should exist per process
// no other instances should exist
// copying these things is probably a bad idea
pub struct Process {
    pub id: u16, // init has id of 1

    pub parent: Option<Box<Process>>, // every process except init should have a parent.
                                         // dangling processes get given to init.
                                         // init must not have a parent.
    // TODO: children

    pub entry_addr: usize,
}

pub fn spawn_init() -> Box<Process> {
    return Box::new(Process {
        id: 1,
        parent: None,
        entry_addr: init_test as usize,
    });
}

#[no_mangle]
fn init_test() {
    uart::write_chars(['I', 'n', 'i', 't', '\n'].into_iter());
}

pub fn context_switch(process: &Process) {
    unsafe {
        // set mstatus so that mret 'returns' to user mode - bits 12-11 determine the privilege level
        // to return to (0b00 for U which we want, 0b01 for S but we don't care about that, 0b11 for M)
        // TODO: that

        // set the trap vector to point to syscall_handle()
        let syscall_handle_addr = syscall_handle as usize;
        asm!("csrw mtvec, {0}", in(reg) syscall_handle_addr);

        // set the return address (ie the 'entry point' for the user-mode code) to be trap_handle
        //let trap_handle_addr = trap_handle as usize;
        let trap_handle_addr = process.entry_addr as usize;
        asm!("csrw mepc, {0}", in(reg) trap_handle_addr);
        // switch to user mode and jump to trap_handle()
        asm!("mret");
        uart::write_word(69);
        uart::write_word(76);
        uart::write_word(76);
        uart::write_word(79);
    }
}

/*#[no_mangle]
unsafe extern "C" fn trap_handle() -> ! {
    /* asm!("li a7, 0");
    asm!("ecall");
    uart::write_word(81);
    loop {}*/
}*/

#[no_mangle]
extern "C" fn syscall_handle() {
    uart::write_chars(['S', 'y', 's', 'c', 'a', 'l', 'l', '\n'].into_iter());
    loop {}
}
