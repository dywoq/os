#![no_main]
#![no_std]

pub(crate) mod panic;

use ::boot::info;
use uefi::Status;
use uefi::boot;
use uefi::entry;
use uefi::helpers;
use uefi::{println, runtime};

#[entry]
fn main() -> Status {
    helpers::init().expect("Failed to initialize helpers");

    // Collect time
    let time = runtime::get_time().expect("Failed to get the time from bootloader");
    let snapshot_time = info::SnapshotTime::from_uefi(time);

    // Put everything inside Info instance
    let info = info::Info {
        snapshot_time: snapshot_time,
    };

    Status::SUCCESS
}
