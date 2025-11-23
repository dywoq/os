// dywoq 2025 - Apache License 2.0
//
// Functions wrappers around assembly instructions
// for AARCH64 target.
//
#ifndef _DYWOQ_OS_KERNEL_ARCH_AARCH64_INSTRUCTIONS_H
#define _DYWOQ_OS_KERNEL_ARCH_AARCH64_INSTRUCTIONS_H

namespace dywoq::os::kernel::arch::instructions
{
    inline void
    relax() noexcept
    {
        __asm__ volatile("pause" : : :);
    }

    inline void
    halt() noexcept
    {
        __asm__ volatile("hlt" : : :);
    }

    inline void
    irqs_on() noexcept
    {
        __asm__ volatile("sti" : : :);
    }

    inline void
    irqs_off() noexcept
    {
        __asm__ volatile("cli" : : :);
    }
} // namespace dywoq::os::kernel::arch::instructions

#endif