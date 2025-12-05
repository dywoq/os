/// A time received from the bootloader.
pub struct SnapshotTime(u16, u8, u8, u8, u8, u8, u32, Option<i16>);

impl SnapshotTime {
    /// Copies the time from UEFI time structure,
    /// stores it in SnapshotTime structure and returns it.
    pub fn from_uefi(time: uefi::runtime::Time) -> SnapshotTime {
        SnapshotTime(
            time.year(),
            time.month(),
            time.day(),
            time.hour(),
            time.minute(),
            time.second(),
            time.nanosecond(),
            time.time_zone(),
        )
    }

    /// Returns the year.
    pub fn year(&self) -> u16 {
        self.0
    }

    /// Returns the month.
    pub fn month(&self) -> u8 {
        self.1
    }

    /// Returns the day.
    pub fn day(&self) -> u8 {
        self.2
    }

    /// Returns the hour.
    pub fn hour(&self) -> u8 {
        self.3
    }

    /// Returns the minute.
    pub fn minute(&self) -> u8 {
        self.4
    }

    /// Returns the second.
    pub fn second(&self) -> u8 {
        self.5
    }

    /// Returns the nanosecond.
    pub fn nanosecond(&self) -> u32 {
        self.6
    }

    /// Returns the timezone.
    pub fn time_zone(&self) -> Option<i16> {
        self.7
    }
}
