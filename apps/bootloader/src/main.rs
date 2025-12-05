#![no_main]
#![no_std]

pub(crate) mod panic;

use ::boot::info;
use uefi::Status;
use uefi::boot;
use uefi::boot::get_handle_for_protocol;
use uefi::entry;
use uefi::helpers;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::PixelFormat;
use uefi::{println, runtime};

#[entry]
fn main() -> Status {
    helpers::init().expect("Failed to initialize helpers");

    // Collect time
    let time = runtime::get_time().expect("Failed to get the time from bootloader");
    let snapshot_time = info::SnapshotTime::from_uefi(time);

    // Collect framebuffer
    let gop_handle = get_handle_for_protocol::<GraphicsOutput>().expect("GOP not found");
    let mut gop =
        boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).expect("Failed to open GOP");
    let framebuffer = info::Framebuffer::from_uefi(&mut gop);

    // Put everything inside Info instance
    let info = info::Info {
        snapshot_time: snapshot_time,
        framebuffer: framebuffer,
    };

    Status::SUCCESS
}
