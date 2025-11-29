#include "../inc/kdebug.h"
#include "../inc/kbuild.h"

void
KeDebugAssert(IN BOOL condition, IN OPTIONAL KE_DEBUG_INFO *Info)
{
    if (KeGetBuildType() != KE_BUILD_TYPE_DEBUG)
    {
        return;
    }

    if (Info == nullptr)
    {
        // do nothing until we have way to free resources and terminating OS
        return;
    }

    // see comment higher
    return;
}
