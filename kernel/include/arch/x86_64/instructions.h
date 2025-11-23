// dywoq 2025 - Apache License 2.0
//
// Functions wrappers around assembly instructions
// for x86_64 target.
//
#ifndef _DYWOQ_OS_KERNEL_ARCH_X86_64_INSTRUCTIONS_H
#define _DYWOQ_OS_KERNEL_ARCH_X86_64_INSTRUCTIONS_H

namespace dywoq::os::kernel::arch::instructions
{
    inline void
    relax() noexcept
    {
        __asm__ volatile("yield" : : : "memory");
    }

    inline void
    halt() noexcept
    {
        __asm__ volatile("wfi" : : :);
    }

    inline void
    irqs_on() noexcept
    {
        __asm__ volatile("msr daifclr, #2" : : :);
    }

    inline void
    irqs_off() noexcept
    {
        __asm__ volatile("msr daifset, #2" : : :);
    }
} // namespace dywoq::os::kernel::arch::instructions

#endif