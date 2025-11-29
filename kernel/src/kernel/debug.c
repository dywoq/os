#include "../../inc/kernel/debug.h"
#include "../../inc/kernel/build.h"

void
KeDebugAssert(IN BOOL condition, IN OPTIONAL KE_DEBUG_INFO *info)
{
    if (KeGetBuildType() != KE_BUILD_TYPE_DEBUG)
    {
        return;
    }

    if (info == 0)
    {
        // do nothing until we have way to free resources and terminating OS
        return;
    }

    if (!condition)
    {
        // see comment higher
        return;
    }
}

const char *
KeBuildTypeToString(KE_BUILD_TYPE type)
{
    switch (type)
    {
    case KE_BUILD_TYPE_DEBUG:
        return "debug";
    case KE_BUILD_TYPE_RELEASE:
        return "release";
    }
}
