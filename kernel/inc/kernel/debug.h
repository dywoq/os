/**
 * dywoq 2025
 *
 * Header name:
 * 	kernel/debug.h
 *
 * Abstract:
 * 	Functionality related to the kernel debugging.
 *
 * Author:
 * 	dywoq - https://github.com/dywoq
 *
 */
#pragma once

#include "../def/doc.h"
#include "../def/base.h"

/**
 * Optional debugging information.
 */
typedef struct _KE_DEBUG_INFO
{
    const char *Message;
    const char *FileName;
    UINT Line;
} KE_DEBUG_INFO;

/**
 * Initializes KE_DEBUG_INFO, but with built-in filename and line.
 */
#define KE_DEBUG_INFO_STANDARD(Message)                                                                                \
    _KE_DEBUG_INFO                                                                                                     \
    {                                                                                                                  \
        Message, __FILE__, __LINE__                                                                                    \
    }

/**
 * Checks if the condition is true.
 * If it's not, it immediately terminates the OS process, freeing resources.
 * The function prints the information if it's not null.
 * 
 * KeDebugAssert will only work if build type is set to debug.
 */
void
KeDebugAssert(IN BOOL condition, IN OPTIONAL KE_DEBUG_INFO *Info);
