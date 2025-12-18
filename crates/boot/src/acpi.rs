use core::ffi;

#[repr(C)]
pub struct Acpi {
    pub address: *const ffi::c_void,
}

impl Acpi {
    /// Copies the ACPI address, received from UEFI bootloader.
    pub fn from_uefi(address: *const ffi::c_void) -> Acpi {
        Acpi { address }
    }
}
