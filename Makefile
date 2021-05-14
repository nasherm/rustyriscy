TARGET=target/riscv64gc-unknown-none-elf/debug/rustyriscy
debug:
		riscv-linux-gnugdb $(TARGET) -iex 'set architecture riscv:rv64' -iex 'target remote localhost:1234'

