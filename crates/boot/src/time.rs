use uefi::runtime;

/// Received time from the bootloader.
#[repr(C)]
pub struct SnapshotTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32
}

impl SnapshotTime {
    /// Copies the time from the UEFI structure [uefi::runtime::Time],
    /// and stores it in [SnapshotTime], returning it.
    pub fn from_uefi(t: &runtime::Time) -> SnapshotTime {
        SnapshotTime { 
            year: t.year(), 
            month: t.month(), 
            day: t.day(), 
            hour: t.hour(), 
            minute: t.minute(), 
            second: t.second(), 
            nanosecond: t.nanosecond() 
        }
    }
}

