#![no_std]
#![no_main]

use core::{ time::Duration};
use uefi::{Status, boot, entry, println};

pub(crate) mod panic;

#[entry]
fn main() -> Status {
    println!("Hello from bootloader");
    boot::stall(Duration::from_secs(23));
    Status::SUCCESS
}
