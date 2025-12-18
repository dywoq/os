use uefi::proto::console;

#[repr(u8)]
pub enum PixelFormat {
    Rgb = 0,
    Bgr = 1,
    Bitmask = 2,
    BltOnly = 3,
}

impl PixelFormat {
    /// Translates the UEFI pixel format enum into [PixelFormat].
    pub fn from_uefi(pixel_format: console::gop::PixelFormat) -> PixelFormat {
        match pixel_format {
            console::gop::PixelFormat::Rgb => PixelFormat::Rgb,
            console::gop::PixelFormat::Bgr => PixelFormat::Bgr,
            console::gop::PixelFormat::Bitmask => PixelFormat::Bitmask,
            console::gop::PixelFormat::BltOnly => PixelFormat::BltOnly,
        }
    }
}

#[repr(C)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub address: *mut u8,
    pub pixel_format: PixelFormat,
}

impl FrameBuffer {
    /// Copies the graphics information from the UEFI structure,
    /// storing it in [FrameBuffer] and returns it.
    pub fn from_uefi(gop: &mut console::gop::GraphicsOutput) -> FrameBuffer {
        let mode = gop.current_mode_info();
        let (width, height) = mode.resolution();
        let fb = gop.frame_buffer().as_mut_ptr();
        let pixel_format = PixelFormat::from_uefi(mode.pixel_format());

        FrameBuffer {
            width: width,
            height: height,
            address: fb,
            pixel_format: pixel_format,
        }
    }
}
