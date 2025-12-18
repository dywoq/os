#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::{Status, entry, helpers, println};

pub(crate) mod info;

/// Custom panic handler.
///
/// If you get LSP error about duplicate `panic_impl` implementation,
/// you may ignore it, it doesn't affect the build.
#[panic_handler]
fn panic_handle(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn init_info() -> boot::Info {
    let snapshot_time = info::get_snapshot_time();
    let framebuffer = info::get_framebuffer();
    let acpi = info::get_acpi();
    boot::Info { framebuffer: framebuffer, acpi: acpi, snapshot_time: snapshot_time }
}

#[entry()]
fn main() -> Status {
    helpers::init().unwrap();
    
    let info = init_info();

    println!("Hello, world!");
    Status::SUCCESS
}
