PROJECT_NAME	:= mos
QEMU	:=  qemu-system-riscv64
BOOT_LOADER	:= ./bootloader/rustsbi-qemu.bin
BIN_FILE	:= ./target/riscv64gc-unknown-none-elf/release/$(PROJECT_NAME)
BASE_ADDR	:= 0x80200000
QEMU_FLAGS	:= -machine virt -nographic   -bios $(BOOT_LOADER) -device loader,file=$(BIN_FILE),addr=$(BASE_ADDR)
QEMU_DBG_FLAGS	:= -s -S 


.PHONY: clean,all,run

all:
	cargo build --release 

run:all
	$(QEMU) $(QEMU_FLAGS) 
clean: 
	@rm -rf ./target - 


