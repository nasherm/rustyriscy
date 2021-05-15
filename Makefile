TRIPLE=riscv64gc-unknown-none-elf
TARGET=target/$(TRIPLE)/debug/rustyriscy
QEMU_BASE_COMMAND = qemu-system-riscv64
QEMU_FLAGS=   -machine virt
QEMU_FLAGS+=  -cpu rv64
QEMU_FLAGS+=  -smp 4
QEMU_FLAGS+=  -m 128M
QEMU_FLAGS+=  -serial mon:stdio
QEMU_FLAGS+=  -d guest_errors,unimp
QEMU_FLAGS+=  -drive if=none,format=raw,file=hdd.dsk,id=foo
QEMU_FLAGS+=  -bios none
QEMU_FLAGS+=  -device virtio-rng-device
QEMU_FLAGS+=  -device virtio-gpu-device
QEMU_FLAGS+=  -device virtio-net-device
QEMU_FLAGS+=  -device virtio-tablet-device
QEMU_FLAGS+=  -device virtio-keyboard-device
QEMU_FLAGS+=  -kernel $(TARGET)
QEMU_FLAGS+=  -s -S -nographic

target:
	cargo build

.PHONY: target

debug: target 
	$(QEMU_BASE_COMMAND) $(QEMU_FLAGS) > a.log 2>&1 &
	riscv64-unknown-elf-gdb $(TARGET) -iex 'set architecture riscv:rv64' -iex 'target remote localhost:1234'

