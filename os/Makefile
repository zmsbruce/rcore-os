TARGET := riscv64gc-unknown-none-elf
MODE := release
TARGET_DIR := target/$(TARGET)/$(MODE)
OBJCOPY := rust-objcopy --binary-architecture=riscv64

all: binary

elf:
	cargo build --$(MODE)
	
binary: elf
	$(OBJCOPY) $(TARGET_DIR)/os --strip-all -O binary $(TARGET_DIR)/os.bin

clean:
	rm -rvf target Cargo.lock

.PHONY: all elf binary clean