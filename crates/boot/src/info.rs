use core::slice;

use uefi::{
    proto::console::{self, gop::GraphicsOutput},
    runtime::Time,
};

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

/// The format of the pixels in a framebuffer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb = 0,
    Bgr,
    BitMask,
    BitOnly,
}

/// Represents the pixels displayed on the screen.
pub struct Framebuffer<'a>(&'a mut [u8], usize, usize, PixelFormat, usize);

impl<'a> Framebuffer<'a> {
    pub fn from_uefi(gop: &mut GraphicsOutput) -> Framebuffer<'_> {
        let info = gop.current_mode_info();

        // Decide the pixel format
        let pixel_format = match info.pixel_format() {
            console::gop::PixelFormat::Rgb => PixelFormat::Rgb,
            console::gop::PixelFormat::Bgr => PixelFormat::Bgr,
            console::gop::PixelFormat::Bitmask => PixelFormat::BitMask,
            console::gop::PixelFormat::BltOnly => PixelFormat::BitOnly,
        };

        // Get resolution
        let (width, height) = info.resolution();

        // Translate the framebuffer address into a slice for safe Rust operation
        let ptr = gop.frame_buffer().as_mut_ptr();
        let ptr_size = gop.frame_buffer().size();
        let slice = unsafe { slice::from_raw_parts_mut(ptr, ptr_size) };

        // Finally, get the number of pixels per scanline
        let stride = info.stride();

        Framebuffer(slice, width, height, pixel_format, stride)
    }

    pub fn buffer(&self) -> &[u8] {
        self.0
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.0
    }

    pub fn width(&self) -> usize {
        self.1
    }

    pub fn height(&self) -> usize {
        self.2
    }

    pub fn pixel_format(&self) -> PixelFormat {
        self.3
    }

    pub fn stride(&self) -> usize {
        self.4
    }
}

/// All information received from the bootloader.
pub struct Info<'a> {
    pub snapshot_time: SnapshotTime,
    pub framebuffer: Framebuffer<'a>,
}
