#![no_std]

use uefi::runtime::{Time};

/// Holds the last saved time, received from the bootloader.
pub struct SnapshotTime(u16, u8, u8, u8, u8, u8, u32, Option<i16>);

impl SnapshotTime {
    /// Creates a new [`SnapshotTime`] and stores its fields
    /// with information received from the UEFI time params.
    ///
    /// # Example
    ///
    /// ```
    /// let time = uefi::runtime::get_time().expect("failed to get time");
    /// let snapshot_time = SnapshotTime::from_uefi(time);
    /// ```
    pub fn from_uefi(params: Time) -> SnapshotTime {
        SnapshotTime(
            params.year(),
            params.month(),
            params.day(),
            params.hour(),
            params.minute(),
            params.second(),
            params.nanosecond(),
            params.time_zone(),
        )
    }

    /// Returns the year of this [`SnapshotTime`].
    pub fn year(&self) -> u16 {
        self.0
    }

    /// Returns the month of this [`SnapshotTime`].
    pub fn month(&self) -> u8 {
        self.1
    }

    /// Returns the day of this [`SnapshotTime`].
    pub fn day(&self) -> u8 {
        self.2
    }

    /// Returns the hour of this [`SnapshotTime`].
    pub fn hour(&self) -> u8 {
        self.3
    }

    /// Returns the minute of this [`SnapshotTime`].
    pub fn minute(&self) -> u8 {
        self.4
    }

    /// Returns the second of this [`SnapshotTime`].
    pub fn second(&self) -> u8 {
        self.5
    }

    /// Returns the nanosecond of this [`SnapshotTime`].
    pub fn nanosecond(&self) -> u32 {
        self.6
    }

    /// Returns the time zone of this [`SnapshotTime`].
    pub fn time_zone(&self) -> Option<i16> {
        self.7
    }
}
