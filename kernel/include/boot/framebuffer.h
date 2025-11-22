/**
   dywoq 2025 - Apache License 2.0

   boot/framebuffer.h

   Abstract:
      Framebuffer declarations

    Author:
      dywoq - https://github.com/dywoq
*/
#ifndef _DYWOQ_OS_KERNEL_BOOT_FRAMEBUFFER_H
#define _DYWOQ_OS_KERNEL_BOOT_FRAMEBUFFER_H

#include "../types.h"

namespace dywoq::os::kernel::boot
{
    //
    // Abstract:
    //  Represents the pixel format of the framebuffer, normalized for kernel use.
    //
    enum class pixel_format : uint32_t
    {
        rgb,
        bgr,
        bitmask,
        reserved
    };

    //
    // Abstract:
    //  Contains information about the framebuffer provided by the firmware.
    //  All sizes are in bytes unless otherwise noted.
    //
    struct framebuffer
    {
        void    *base;
        uint64_t size_bytes;

        uint32_t width;
        uint32_t height;

        uint32_t pitch_bytes;

        pixel_format format;
    };
} // namespace dywoq::os::kernel::boot

#endif