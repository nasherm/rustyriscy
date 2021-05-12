debug:
		gdb -iex 'set architecture riscv:rv64' -iex 'target remote localhost:1234'

