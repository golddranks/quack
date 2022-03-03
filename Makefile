test.musl.elf: src/test_elf.c
	zig cc -target x86_64-linux-musl src/test_elf.c -o test.musl.elf

test.gnu.elf: src/test_elf.c
	zig cc -target x86_64-linux-gnu src/test_elf.c -o test.gnu.elf

