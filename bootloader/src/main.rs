#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::{Status, entry, helpers, println};

/// Custom panic handler.
/// 
/// If you get LSP error about duplicate `panic_impl` implementation,
/// you may ignore it, it doesn't affect the build.
#[panic_handler]
fn panic_handle(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[entry()]
fn main() -> Status {
    helpers::init().unwrap();
    println!("Hello, world!");
    Status::SUCCESS
}

