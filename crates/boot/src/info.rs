use core::{ffi::c_void, slice};

use uefi::{
    boot,
    mem::{self, memory_map::MemoryMap},
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

/// Represents the address of the ACPI table provided by the bootloader.
/// Used to access ACPI information in the kernel.
pub struct Acpi {
    pub address: *const c_void,
}

impl Acpi {
    pub fn from_uefi(address: *const c_void) -> Acpi {
        Acpi { address }
    }
}

pub enum MemoryType {
    Reserved = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    Conventional = 7,
    Unusable = 8,
    AcpiReclaim = 9,
    AcpiNonVolatile = 10,
    Mmio = 11,
    MmioPortSpace = 12,
    PalCode = 13,
    PersistentMemory = 14,
    Unaccepted = 15,
    Max = 16,
}

impl MemoryType {
    /// Converts the `uefi::boot::MemoryType`
    /// into `boot::info::MemoryType`.
    ///
    /// This is the UEFI version of this function.
    pub fn from_uefi(mt: boot::MemoryType) -> MemoryType {
        match mt.0 {
            0 => MemoryType::Reserved,
            1 => MemoryType::LoaderCode,
            2 => MemoryType::LoaderData,
            3 => MemoryType::BootServicesCode,
            4 => MemoryType::BootServicesData,
            7 => MemoryType::Conventional,
            8 => MemoryType::Unusable,
            9 => MemoryType::AcpiReclaim,
            10 => MemoryType::AcpiNonVolatile,
            11 => MemoryType::Mmio,
            12 => MemoryType::MmioPortSpace,
            13 => MemoryType::PalCode,
            14 => MemoryType::PersistentMemory,
            15 => MemoryType::Unaccepted,
            16 => MemoryType::Max,
            _ => MemoryType::Reserved,
        }
    }
}

pub struct MemoryEntry {
    pub memory_type: MemoryType,
    pub phys_start: u64,
    pub virt_start: u64,
    pub page_count: u64,
}

impl MemoryEntry {
    /// Copies the `uefi::boot::MemoryDescriptor` fields
    /// and stores it in MemoryEntry.
    ///
    /// This is the UEFI version of this function .
    pub fn from_uefi(descriptor: &boot::MemoryDescriptor) -> MemoryEntry {
        MemoryEntry {
            memory_type: MemoryType::from_uefi(descriptor.ty),
            phys_start: descriptor.phys_start,
            virt_start: descriptor.virt_start,
            page_count: descriptor.page_count,
        }
    }
}

pub struct MMemoryMap {
    pub entries: *const MemoryEntry,
    pub entry_count: usize,
}

impl MMemoryMap {
    /// Creates a MMemoryMap from UEFI memory map.
    /// The entries buffer must be pre-allocated with sufficient size.
    ///
    /// # Safety
    /// The caller must ensure that `entries_buffer` has enough space
    /// to hold all memory map entries.
    pub unsafe fn from_uefi(
        _map: &mem::memory_map::MemoryMapOwned,
        entries_buffer: *mut MemoryEntry,
    ) -> MMemoryMap {
        let mut index = 0;
        for entry in _map.entries() {
            // Write to the buffer
            unsafe {
                entries_buffer
                    .add(index)
                    .write(MemoryEntry::from_uefi(entry))
            };
            index += 1;
        }

        MMemoryMap {
            entries: entries_buffer,
            entry_count: index,
        }
    }

    /// Get the entries as a slice
    pub fn entries(&self) -> &[MemoryEntry] {
        unsafe { slice::from_raw_parts(self.entries, self.entry_count) }
    }
}

/// All information received from the bootloader.
pub struct Info<'a> {
    pub snapshot_time: SnapshotTime,
    pub framebuffer: Framebuffer<'a>,
    pub acpi: Acpi,
    pub memory_map: MMemoryMap,
}
