PROJECT_NAME	:= mos
QEMU	:=  qemu-system-riscv64
OBJCOPY	:= rust-objcopy
BOOT_LOADER	:= ./bootloader/rustsbi-qemu.bin
TARGET_FILE	:= ./target/riscv64gc-unknown-none-elf/release/$(PROJECT_NAME)
BIN_FILE 	:= $(TARGET_FILE).bin
BASE_ADDR	:= 0x80200000
OBJCOPY_FLAGS	:= --strip-all $(TARGET_FILE)  -O binary $(BIN_FILE)
QEMU_FLAGS	:= -machine virt -nographic   -bios $(BOOT_LOADER) -device loader,file=$(BIN_FILE),addr=$(BASE_ADDR)
QEMU_DBG_FLAGS	:= -s -S 


.PHONY: clean,all,run

all:
	cargo build --release 
	$(OBJCOPY) $(OBJCOPY_FLAGS)
run:all
	$(QEMU) $(QEMU_FLAGS) 
clean: 
	@rm -rf ./target - 


