#![no_std]
#![no_main]

pub mod acpi;
pub mod graphics;
pub mod time;

/// Boot loader information.
#[repr(C)]
pub struct Info {
    pub framebuffer: graphics::FrameBuffer,
    pub acpi: acpi::Acpi,
    pub snapshot_time: time::SnapshotTime,
}

