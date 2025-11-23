// dywoq 2025 - Apache License 2.0
//
// Functions wrappers around assembly instructions
// for the current chosen target.
// Use this header to prevent a lot of macro if-conditions.
//
#pragma once

#include "../detect.h"

#if ARCH_IS_X86_64
#include "../x86_64/instructions.h"
#elif ARCH_IS_AARCH64
#include "../aarch64/instructions.h"
#endif

namespace dywoq::os::kernel::arch::current::instructions
{
    inline void
    relax() noexcept
    {
        arch::instructions::relax();
    }

    inline void
    halt() noexcept
    {
        arch::instructions::halt();
    }

    inline void
    irqs_on() noexcept
    {
        arch::instructions::irqs_on();
    }

    inline void
    irgs_off() noexcept
    {
        arch::instructions::irqs_off();
    }
} // namespace dywoq::os::kernel::arch::current::instructions
