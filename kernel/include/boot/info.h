/**
   dywoq 2025 - Apache License 2.0

   boot/info.h

   Abstract:
      Structure combining all info in one

    Author:
      dywoq - https://github.com/dywoq
*/
#ifndef _DYWOQ_OS_KERNEL_BOOT_INFO_H
#define _DYWOQ_OS_KERNEL_BOOT_INFO_H

#include "../types.h"

#include "acpi.h"
#include "framebuffer.h"
#include "memory.h"

namespace dywoq::os::kernel::boot
{

    //
    // Abstract:
    //  Aggregates all sanitized boot information for the kernel.
    //  Includes memory map, framebuffer, ACPI info, and runtime service state.
    //
    struct boot_info
    {
        memory_map  memory;
        framebuffer fb;
        acpi_info   acpi;

        bool  runtime_services_available;
        void *kernel_load_base;
    };
} // namespace dywoq::os::kernel::boot

#endif