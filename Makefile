

dbg:
	gdb-multiarch -ex "file target/mips32/debug/mos-6502" -ex "target remote localhost:1234"

dbg-run:
	qemu-system-mipsel -M malta -kernel target/mips32/debug/mos-6502 -nographic -s -S

objdump:
	mips-linux-gnu-objdump -dS target/mips32/debug/mos-6502 > 1