//
// dywoq 2025 - Apache License 2.0
//
// boot.h
//
// Abstract:
//  This header contains necessary declarations,
//  used to provide boot info from bootloader to the kernel entry.
//
// WARNING:
//  boot.h must contain only definitions and declarations,
//  not functions.
//
// Author:
//  dywoq - https://github.com/dywoq
//
#ifndef _DYWOQOS_BOOT_H
#define _DYWOQOS_BOOT_H

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct boot_framebuffer
    {
        void              *address;
        unsigned long long size;
        unsigned int       screen_width;
        unsigned int       screen_height;
        unsigned int       pixels_per_scanline;
        unsigned int       pixel_format;
    } boot_framebuffer_t;

    typedef struct boot_memory
    {
        void         *map;
        unsigned long map_size;
        unsigned long descriptor_size;
        unsigned int  map_version;
    } boot_memory_t;

    typedef struct boot_info
    {
        boot_framebuffer_t *framebuffer;
        boot_memory_t      *memory;
        void               *rsdp;
    } boot_info_t;

#ifdef __cplusplus
}
#endif

#endif