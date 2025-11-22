/**
   dywoq 2025 - Apache License 2.0

   boot/memory.h

   Abstract:
      Boot memory declarations

    Author:
      dywoq - https://github.com/dywoq
*/
#ifndef _DYWOQ_OS_KERNEL_BOOT_MEMORY_H
#define _DYWOQ_OS_KERNEL_BOOT_MEMORY_H

#include <efi/efi.h>
#include <efi/efiapi.h>
#include <efi/efidef.h>

#include "../types.h"

namespace dywoq::os::kernel::boot
{
    //
    // Abstract:
    //  Describes a single region of physical memory.
    //  The kernel uses this for memory management and runtime mappings.
    //
    struct memory_region
    {
        uint64_t base;
        uint64_t length;

        bool usable;
        bool runtime;
        bool mmio;
    };

    //
    // Abstract:
    //  Represents the entire memory map for the kernel.
    //  Includes total usable memory and highest physical address.
    //
    struct memory_map
    {
        memory_region* regions;

        uint32_t region_count;

        uint64_t total_usable;
        uint64_t highest_address;
    };
} // namespace dywoq::os::kernel::boot

#endif