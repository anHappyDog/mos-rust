PROJECT_NAME	:= mos
QEMU	:=  qemu-system-riscv64
OBJCOPY	:= rust-objcopy
BOOT_LOADER	:= ./bootloader/rustsbi-qemu.bin
TARGET_DIR	:= ./target/riscv64gc-unknown-none-elf/release
TARGET_FILE	:= $(TARGET_DIR)/$(PROJECT_NAME)
BIN_FILE 	:= $(TARGET_FILE).bin
OBJDUMP_FILE	:= $(TARGET_FILE).objdump
BASE_ADDR	:= 0x80200000
OBJCOPY_FLAGS	:= --strip-all $(TARGET_FILE)  -O binary $(BIN_FILE)
QEMU_FLAGS	:= -machine virt -nographic   -bios $(BOOT_LOADER) -device loader,file=$(BIN_FILE),addr=$(BASE_ADDR)
QEMU_DBG_FLAGS	:= -s -S 
GDB		:= riscv64-unknown-elf-gdb
GDB_FLAGS	:= -ex "target/riscv64gc-unknown-none-elf/release/$(BIN_FILE)" -ex "set arch riscv:rv64" -ex "target remote localhost:1234"
.PHONY: clean,all,run

all:
	cargo build --release 
	$(OBJCOPY) $(OBJCOPY_FLAGS)
run:all
	$(QEMU) $(QEMU_FLAGS) 

dbg: all
	$(QEMU) $(QEMU_FLAGS) $(QEMU_DBG_FLAGS)
dbg-run:
	$(GDB) $(GDB_FLAGS)

objdump: all
	cargo objdump --release -- -d > $(OBJDUMP_FILE)
	vim $(OBJDUMP_FILE)
clean: 
	@rm -rf ./target - 


