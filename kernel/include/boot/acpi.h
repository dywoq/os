/**
   dywoq 2025 - Apache License 2.0

   boot/acpi.h

   Abstract:
      Declarations related to ACPI

    Author:
      dywoq - https://github.com/dywoq
*/
#ifndef _DYWOQ_OS_KERNEL_BOOT_ACPI_H
#define _DYWOQ_OS_KERNEL_BOOT_ACPI_H

#include "../types.h"

namespace dywoq::os::kernel::boot
{
    //
    // Abstract:
    //  Contains ACPI information, including validated RSDP pointer and version info.
    //
    struct acpi_info
    {
        void *rsdp;
        bool  xsdt;
    };
} // namespace dywoq::os::kernel::boot

#endif