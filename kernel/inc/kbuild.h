/**
 * dywoq 2025
 *
 * Header name:
 * 	kbuild.h
 *
 * Abstract:
 * 	Functionality to help you see the information of build 
 * 	more comfortable.
 *
 * Author:
 * 	dywoq - https://github.com/dywoq
 */
#pragma once

typedef enum _KE_BUILD_TYPE {
	KE_BUILD_TYPE_DEBUG,
	KE_BUILD_TYPE_RELEASE
} KE_BUILD_TYPE;

/**
 * Returns the build type of the OS.
 * May return KE_BUILD_TYPE_DEBUG or KE_BUILD_TYPE_RELEASE.
 */
KE_BUILD_TYPE
KeGetBuildType();
