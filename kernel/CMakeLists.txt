cmake_minimum_required(VERSION 3.28.3)

project(MyKernel ASM C CXX)

set(CMAKE_C_COMPILER clang-21)
set(CMAKE_CXX_COMPILER clang++-21)
set(CMAKE_ASM_COMPILER clang-21)

set(ARCH "x86_64")

set(UEFI_C_FLAGS
    -target x86_64-unknown-windows
    -ffreestanding
    -fno-stack-protector
    -fno-stack-check
    -fshort-wchar
    -mno-red-zone
    -mno-sse
    -mno-sse2
    -Wall
    -Wextra
)

set(UEFI_LINK_FLAGS
    -target x86_64-unknown-windows
    -nostdlib
    -Wl,-entry:efi_main
    -Wl,-subsystem:efi_application
    -fuse-ld=lld
)

file(GLOB_RECURSE KERNEL_SOURCES
    "src/*.c"
    "src/*.cxx"
    "src/*.s"
)

file(GLOB_RECURSE BOOTLOADER_SOURCES
    "boot/*.c"
)

add_executable(kernel.elf
    ${KERNEL_SOURCES}
    ${BOOTLOADER_SOURCES}
)

target_compile_options(kernel.elf PRIVATE ${UEFI_C_FLAGS})

target_link_options(kernel.elf PRIVATE
    ${UEFI_LINK_FLAGS}
    -T ${CMAKE_SOURCE_DIR}/linker.ld
)

# Convert ELF to PE32+ (UEFI executable format)
add_custom_command(TARGET kernel.elf POST_BUILD
    COMMAND llvm-objcopy
    ARGS
        -O binary
        $<TARGET_FILE:kernel.elf>
        ${CMAKE_BINARY_DIR}/BOOTX64.EFI
    COMMENT "Converting ELF to UEFI PE32+ executable"
)

add_custom_target(image
    COMMAND mkdir -p ${CMAKE_BINARY_DIR}/iso/EFI/BOOT
    COMMAND cp ${CMAKE_BINARY_DIR}/BOOTX64.EFI ${CMAKE_BINARY_DIR}/iso/EFI/BOOT/
    DEPENDS bootx64
    COMMENT "Creating bootable directory structure"
)

set_directory_properties(PROPERTIES
    ADDITIONAL_CLEAN_FILES "${CMAKE_BINARY_DIR}/BOOTX64.EFI;${CMAKE_BINARY_DIR}/iso"
)