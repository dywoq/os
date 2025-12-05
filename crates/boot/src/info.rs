use uefi::runtime::Time;

/// A snapshot of time received from the bootloader and
/// given to the kernel.
pub struct SnapshotTime(u16, u8, u8, u8, u8, u8, u32, Option<i16>);

impl SnapshotTime {
    pub fn from_uefi(t: Time) -> SnapshotTime {
        SnapshotTime(
            t.year(),
            t.month(),
            t.day(),
            t.hour(),
            t.minute(),
            t.second(),
            t.nanosecond(),
            t.time_zone(),
        )
    }

    pub fn year(&self) -> u16 {
        self.0
    }

    pub fn month(&self) -> u8 {
        self.1
    }

    pub fn day(&self) -> u8 {
        self.2
    }

    pub fn hour(&self) -> u8 {
        self.3
    }

    pub fn minute(&self) -> u8 {
        self.4
    }

    pub fn second(&self) -> u8 {
        self.5
    }

    pub fn nanosecond(&self) -> u32 {
        self.6
    }

    pub fn time_zone(&self) -> Option<i16> {
        self.7
    }
}
