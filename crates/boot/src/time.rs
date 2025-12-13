use uefi::runtime::Time;

/// A stored snapshot time retrieved from UEFI bootloader.
pub struct SnapshotTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
    pub time_zone: Option<i16>,
}

impl SnapshotTime {
    /// Creates a new [SnapshotTime], copying the information 
    /// from given [uefi::runtime::Time].
    /// 
    /// Use it for UEFI bootloader.
    pub fn from_uefi(t: &Time) -> SnapshotTime {
        SnapshotTime {
            year: t.year(),
            month: t.month(),
            day: t.day(),
            hour: t.hour(),
            minute: t.minute(),
            second: t.second(),
            nanosecond: t.nanosecond(),
            time_zone: t.time_zone(),
        }
    }
}
