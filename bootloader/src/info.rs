use core::{ffi, ptr};

use uefi::{proto::console::gop::GraphicsOutput, runtime, system, table::cfg};

pub fn get_snapshot_time() -> boot::time::SnapshotTime {
    let time = runtime::get_time().unwrap();
    let snapshot_time = boot::time::SnapshotTime::from_uefi(&time);
    snapshot_time
}

pub fn get_acpi() -> boot::acpi::Acpi {
    let mut address: *const ffi::c_void = ptr::null();
    system::with_config_table(|table| {
        for elem in table {
            if elem.guid == cfg::ConfigTableEntry::ACPI2_GUID
                || elem.guid == cfg::ConfigTableEntry::ACPI_GUID
            {
                address = elem.address;
                break;
            }
        }
    });

    if address.is_null() {
        panic!("failed to get ACPI address");
    } else {
        boot::acpi::Acpi { address: address }
    }
}

pub fn get_framebuffer() -> boot::graphics::FrameBuffer {
    let gop_handle = uefi::boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = uefi::boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();
    let fb = boot::graphics::FrameBuffer::from_uefi(&mut gop);
    fb
}
