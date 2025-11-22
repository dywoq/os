cmake_minimum_required(VERSION 3.28.3)
project(dywoqos LANGUAGES C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-exceptions -fno-rtti -Wall -Wextra -Wpedantic")

set(EFI_TARGET x86_64)
set(EFI_OUTPUT_DIR ${CMAKE_BINARY_DIR}/efi)

add_subdirectory(kernel)

set(ISO_DIR ${CMAKE_BINARY_DIR}/iso)
set(ISO_OUTPUT ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/DywoqOS.iso)

add_custom_target(iso ALL
    COMMAND ${CMAKE_COMMAND} -E make_directory ${ISO_DIR}/EFI/BOOT
    COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:bootloader.efi> ${ISO_DIR}/EFI/BOOT/BOOTX64.EFI
    COMMAND xorriso -as mkisofs \
        -iso-level 3 \
        -volid DYWOQ_OS \
        -output ${ISO_OUTPUT} \
        -efi-boot-part \
        -efi-boot-image \
        -no-emul-boot \
        -isohybrid-gpt-basdat \
        ${ISO_DIR}
    DEPENDS bootloader.efi kernel.elf
    COMMENT "Building UEFI bootable ISO"
)
