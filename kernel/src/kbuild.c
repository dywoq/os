#include "../inc/kernel/build.h"

KE_BUILD_TYPE
KeGetBuildType()
{
#if BUILD_RELEASE
    return KE_BUILD_TYPE_RELEASE;
#elif BUILD_DEBUG
    return KE_BUILD_TYPE_DEBUG;
#endif
}
