#![no_main]
#![no_std]

pub(crate) mod panic;

use core::ptr::null;
use core::{ffi::c_void, ptr::*};

use ::boot::info::{self, MemoryEntry};
use uefi::mem::memory_map::MemoryMap;
use uefi::{
    Status,
    boot::{self, MemoryType, get_handle_for_protocol},
    entry, helpers,
    proto::console::gop::GraphicsOutput,
    runtime, system,
    table::cfg::ConfigTableEntry,
};

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

    // Collect memory map BEFORE exiting boot services
    let memory_map_owned =
        boot::memory_map(MemoryType::LOADER_DATA).expect("Failed to get memory map");

    let entry_count = memory_map_owned.entries().count();
   
    let buffer_size = entry_count * core::mem::size_of::<MemoryEntry>();
    let buffer_ptr = boot::allocate_pool(MemoryType::LOADER_DATA, buffer_size)
        .expect("Failed to allocate memory map buffer");
    let buffer: *mut MemoryEntry = buffer_ptr.as_ptr() as *mut MemoryEntry;

    let map = unsafe { boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };
    let memory_map = unsafe { info::MMemoryMap::from_uefi(&memory_map_owned, buffer) };

    // Put everything inside Info instance
    let info = info::Info {
        snapshot_time: snapshot_time,
        framebuffer: framebuffer,
        acpi: acpi,
        memory_map: memory_map,
    };

    Status::SUCCESS
}
