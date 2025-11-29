# Config

C := gcc-13
C_FLAGS := -fno-stack-protector -fpic -fshort-wchar -mno-red-zone -ffreestanding -Wall -Wextra --std=c23

BUILD_DIR := build
OBJ_DIR := $(BUILD_DIR)/objs

# Sources
SOURCES := $(wildcard src/*.c)

KERNEL_SOURCES := $(wildcard src/kernel/*.c)
SOURCES += KERNEL_SOURCES

OBJECTS := $(patsubst src/%.c,$(OBJ_DIR)/%.o,$(SOURCES))

TARGET := dywoqos.efi

# Detect build type
BUILD_TYPE ?= debug
BUILD_TYPES_VALID := debug release

ifeq ($(filter $(BUILD_TYPE),$(BUILD_TYPES_VALID)),)
$(error Invalid BUILD_TYPE: $(BUILD_TYPE). Allowed: $(BUILD_TYPES_VALID))
endif

ifeq (BUILD_TYPE,debug) 
C_FLAGS += -D -DBUILD_RELEASE=0
C_FLAGS += -D -DBUILD_DEBUG=1
endif

ifeq (BUILD_TYPE,release) 
C_FLAGS += -D -DBUILD_RELEASE=1
C_FLAGS += -D -DBUILD_DEBUG=0
endif

# Default target
all: $(BUILD_DIR)/$(TARGET)

# Rule to build .efi from object files
$(BUILD_DIR)/$(TARGET): $(OBJECTS)
	$(C) -nostdlib -znocombreloc -shared -Bsymbolic -T linker.ld -o $@ $^

# Rule to build each object file
$(OBJ_DIR)/%.o: src/%.c | $(OBJ_DIR)
	$(C) $(C_FLAGS) -c $< -o $@

# Create OBJ_DIR if it doesn't exist
$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

# Clean
clean:
	rm -rf $(BUILD_DIR)
