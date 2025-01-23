#![no_std]
use defmt_rtt as _;

#[panic_handler]
fn panic(p: &core::panic::PanicInfo) -> ! {
    defmt::error!(
        "TEST-FAIL: {} failed on line {} with error {}",
        p.location().unwrap().file(),
        p.location().unwrap().line(),
        p.message().as_str()
    );
    cortex_m::asm::bkpt();
    loop {}
}

pub fn pass_test() {
    defmt::info!("TEST-SUCCESS: Example terminated successfully");
    defmt::flush();
    cortex_m::asm::bkpt();
}
