#include "../inc/kbuild.h"

/**
 * Returns the build type of the OS.
 * May return KE_BUILD_TYPE_DEBUG or KE_BUILD_TYPE_RELEASE.
 */
KE_BUILD_TYPE
KeGetBuildType()
{
#if BUILD_RELEASE
    return KE_BUILD_TYPE_RELEASE;
#elif BUILD_DEBUG
    return KE_BUILD_TYPE_DEBUG;
#endif
}
