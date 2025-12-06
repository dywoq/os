#![no_main]
#![no_std]

pub(crate) mod panic;

use core::ffi::c_void;
use core::ptr::null;

use ::boot::info;
use uefi::Status;
use uefi::boot;
use uefi::boot::get_handle_for_protocol;
use uefi::entry;
use uefi::helpers;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::runtime;
use uefi::system;
use uefi::table::cfg::ConfigTableEntry;

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

    // Collect ACPI
    let mut address: *const c_void = null();
    system::with_config_table(|slice| {
        for i in slice {
            match i.guid {
                // We break loop if any of ACPI GUIDs detected
                ConfigTableEntry::ACPI2_GUID | ConfigTableEntry::ACPI_GUID => {
                    address = i.address;
                    break;
                }
                _ => {}
            }
        }
        if address.is_null() {
            panic!("Failed to get the ACPI address");
        }
    });
    let acpi = info::Acpi::from_uefi(address);

    // Put everything inside Info instance
    let info = info::Info {
        snapshot_time: snapshot_time,
        framebuffer: framebuffer,
        acpi: acpi,
    };

    Status::SUCCESS
}
