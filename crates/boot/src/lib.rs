#![no_std]
#![no_main]

pub mod acpi;
pub mod graphics;
pub mod time;

/// Boot loader information.
#[repr(C)]
pub struct Info {
    framebuffer: graphics::FrameBuffer,
    acpi: acpi::Acpi,
    snapshot_time: time::SnapshotTime,
}

