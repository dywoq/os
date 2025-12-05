#![no_main]
#![no_std]

use core::time::Duration;
use uefi::{prelude::*, println};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {} 
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("Hi!");
    boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}
