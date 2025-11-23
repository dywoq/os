/**
    dywoq 2025 - Apache License 2.0

    bootloader.cxx
        UEFI bootloader
*/
#include <efi/efi.h>
#include <efi/efiapi.h>
#include <efi/efiprot.h>

extern "C" EFI_STATUS
get_time() noexcept
{
    EFI_SYSTEM_TABLE            *st;
    EFI_GRAPHICS_OUTPUT_PROTOCOL s;
}
