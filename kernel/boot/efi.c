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
