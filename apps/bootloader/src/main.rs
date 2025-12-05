#![no_main]
#![no_std]

pub(crate) mod panic;

use core::time::Duration;
use uefi::{prelude::*, println};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("Hi!");
	boot::stall(Duration::from_secs(11));
	panic!("error occurred");
    // boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}
