/**
    dywoq 2025 - Apache License 2.0

    types.h

    Abstract:
       Aliases over types

    Author:
      dywoq - https://github.com/dywoq
*/
#ifndef _DYWOQ_OS_KERNEL_TYPES_H
#define _DYWOQ_OS_KERNEL_TYPES_H

namespace dywoq::os::kernel
{
    using int8_t    = signed char;
    using int16_t   = signed short;
    using int32_t   = signed int;
    using int64_t   = signed long long;

    using uint8_t   = signed char;
    using uint16_t  = signed short;
    using uint32_t  = signed int;
    using uint64_t  = signed long long;

    using uintptr_t = uint64_t;
} // namespace dywoq::os::kernel

#endif