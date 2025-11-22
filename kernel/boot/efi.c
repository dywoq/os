//
// dywoq 2025 - Apache License 2.0
//
// efi.c
//
// Abstract:
//  This source file is a bootloader,
//  which initializes boot_info_t and passes to the kernel entry.
//
// Author:
//  dywoq - https://github.com/dywoq
//
#include "boot.h"
#include <efi/efi.h>
#include <efi/efilib.h>
#include <efi/efiprot.h>

//
// Abstract:
//
//  Gets the framebuffer address, its width, height, size etc.
//  and stores it into info.
//
//  May fail if it didn't manage to locate protocol.
//
EFI_STATUS
get_framebuffer(EFI_SYSTEM_TABLE *st, boot_info_t *info)
{
    EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;
    EFI_GUID                      gop_guid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
    EFI_STATUS                    status;

    status = ST->BootServices->LocateProtocol(&gop_guid, NULL, (void **)&gop);
    if (EFI_ERROR(status))
    {

        return status;
    }

    info->framebuffer->address             = (void *)gop->Mode->FrameBufferBase;
    info->framebuffer->size                = gop->Mode->FrameBufferSize;
    info->framebuffer->screen_width        = gop->Mode->Info->HorizontalResolution;
    info->framebuffer->screen_width        = gop->Mode->Info->VerticalResolution;
    info->framebuffer->pixels_per_scanline = gop->Mode->Info->PixelsPerScanLine;
    info->framebuffer->pixel_format        = gop->Mode->Info->PixelFormat;

    return EFI_SUCCESS;
}

//
// Abstract:
//
//  Makes an attempt to get ACPI RSDP and store it into the system table.
//  The function tries to find ACPI 2.0 first, if it fails to do, fallbacks to ACPI 1.0.
//  After all failed attempts, the function outputs an warning.
void
get_rsdp(EFI_SYSTEM_TABLE *st, boot_info_t *info)
{
    EFI_GUID acpi2_guid = ACPI_20_TABLE_GUID;
    EFI_GUID acpi1_guid = ACPI_TABLE_GUID;

    info->rsdp          = NULL;

    // Try ACPI 2.0 first
    for (UINTN i = 0; i < ST->NumberOfTableEntries; i++)
    {
        if (CompareGuid(&ST->ConfigurationTable[i].VendorGuid, &acpi2_guid) == 0)
        {
            info->rsdp = ST->ConfigurationTable[i].VendorTable;
            Print(L"Found ACPI 2.0 RSDP at 0x%lx\n", info->rsdp);
            return;
        }
    }

    // Fallback to ACPI 1.0
    for (UINTN i = 0; i < ST->NumberOfTableEntries; i++)
    {
        if (CompareGuid(&ST->ConfigurationTable[i].VendorGuid, &acpi1_guid) == 0)
        {
            info->rsdp = ST->ConfigurationTable[i].VendorTable;
            Print(L"Found ACPI 1.0 RSDP at 0x%lx\n", info->rsdp);
            return;
        }
    }

    Print(L"WARNING: ACPI RSDP not found.\n");
}

// Abstract:
//
//  Makes an attempt to get the memory map, and exit the boot services.
//  The got memory map stored in info.
//
//  May fail if it fails to allocate pool, get memory map, or exit boot services.
EFI_STATUS
get_memory_map_and_exit(EFI_SYSTEM_TABLE *st, EFI_HANDLE image_handle, boot_info_t *info)
{
    EFI_STATUS status;
    UINTN      map_key;

    info->memory->map_size = 0;
    status                 = st->BootServices->GetMemoryMap(
        &info->memory->map_size, NULL, &map_key, &info->memory->descriptor_size, &info->memory->map_version);

    info->memory->map_size += 2 * info->memory->descriptor_size;
    status = st->BootServices->AllocatePool(EfiLoaderData, info->memory->map_size, &info->memory->map);

    if (EFI_ERROR(status))
    {
        Print(L"failed to allocate memory map: %r\n", status);
        return status;
    }

    status = st->BootServices->GetMemoryMap(
        &info->memory->map_size, (EFI_MEMORY_DESCRIPTOR *)info->memory->map, &map_key, &info->memory->descriptor_size,
        &info->memory->map_version);

    if (EFI_ERROR(status))
    {
        Print(L"failed to get memory map: %r\n", status);
        return status;
    }

    Print(L"memory map obtained, exiting boot services...\n");

    status = st->BootServices->ExitBootServices(image_handle, map_key);
    if (EFI_ERROR(status))
    {
        Print(L"failed to exit boot services: %r\n", status);
        return status;
    }

    return EFI_SUCCESS;
}
