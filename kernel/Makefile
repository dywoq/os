# CONFIG
C := gcc-13
C_FLAGS := -fno-stack-protector -fpic -fshort-wchar -mno-red-zone -ffreestanding -Wall -Wextra

BUILD_DIR := build
OBJ_DIR := $(BUILD_DIR)/objs

SOURCES := $(wildcard src/*.c)
OBJECTS := $(patsubst src/%.c,$(OBJ_DIR)/%.o,$(SOURCES))

TARGET := dywoqos.efi

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
