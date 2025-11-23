// dywoq 2025 - Apache License 2.0
//
// Functions and macros to help you detect the architecture
// more gracefully.
//
#pragma once

#if __x86_64__
#define ARCH_IS_X86_64 1
#else
#define ARCH_IS_X86_64 0
#endif

#if __aarch64__
#define ARCH_IS_AARCH64 1
#else
#define ARCH_IS_AARCH64 0
#endif

namespace dywoq::os::kernel::arch
{
    constexpr bool
    is_x86_64() noexcept
    {
        return ARCH_IS_X86_64;
    }

    constexpr bool
    is_aarch64() noexcept
    {
        return ARCH_IS_AARCH64;
    }
} // namespace dywoq::os::kernel::arch
