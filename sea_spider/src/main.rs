// author: (Hollow.Bones/Chadwick G)
// Inspired by https://os.phil-opp.com
// This is a minimal freestanding binary
// compile with a specific target-triple
// cargo build --target <target-triple>
// to view target-triple use `rustc --version --verbose`
// and look at the host line
#![no_std] // don't link Rust std lib
#![no_main] // disable all Rust-level entry points


use core::panic::PanicInfo;
#[no_mangle] // don't mangle fn name
pub extern "C" fn _start() -> ! {
    // this function is the entry point
    // the linker looks for a fn named `_start` by default`
    loop{}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
