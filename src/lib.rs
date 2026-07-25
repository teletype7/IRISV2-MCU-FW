#![no_main]
#![no_std]
extern crate alloc;

// global logger
use defmt_rtt as _;

// panic handler
use panic_probe as _;

// memory layout
use embassy_stm32 as _;

// memory allocator
use spinning_top::RawSpinlock;
use talc::{TalcLock, source::Claim};

#[global_allocator]
static TALC: TalcLock<RawSpinlock, Claim> = TalcLock::new(unsafe {
    static mut INITIAL_ARENA: [u8; 32768] = [0; 32768];
    Claim::array(&raw mut INITIAL_ARENA)
});

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-rs` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

pub mod icm45686;
pub mod mmc5983;
pub mod bmp581;
