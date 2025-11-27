#![no_std]

use uefi::runtime::TimeParams;

/// Holds the last saved time, received from the bootloader.
pub struct SnapshotTime(
    pub u16,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u32,
    pub Option<i16>,
);

impl SnapshotTime {
    /// Creates a new [`SnapshotTime`] and stores its fields
    /// with information received from the UEFI time params.
    pub fn from_uefi(params: TimeParams) -> SnapshotTime {
        SnapshotTime(
            params.year,
            params.month,
            params.day,
            params.hour,
            params.minute,
            params.second,
            params.nanosecond,
            params.time_zone,
        )
    }
}
