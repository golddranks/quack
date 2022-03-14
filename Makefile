elf: test.musl.elf test.gnu.elf

test.musl.elf: src/test_elf.c
	zig cc -target x86_64-linux-musl -g src/test_elf.c -o test/test.musl.elf

test.gnu.elf: src/test_elf.c
	zig cc -target x86_64-linux-gnu -g src/test_elf.c -o test/test.gnu.elf

clean:
	rm *.elf target/release/quack
