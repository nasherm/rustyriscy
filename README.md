# rustyriscy
A Rust OS targeting RISC-V.

## Dependencies
The following sets up the correct toolchain and target for builds, this assumes you have [rustup install](https://github.com/rust-lang/rustup):

    $ rustup default nightly
    $ rustup target add riscv64gc-unknown-none-elf
    $ cargo install cargo-binutils

The project uses [QEMU](https://www.qemu.org/) as an emulator for a RI64 compliant core. This requires installation

## Testing, Building and Running
To build, call cargo:

    $ cargo build

To run, one can use cargo, but this uses the `runner` specified in .cargo/config.toml. The Makefile can be used to have more fine 
grained control of flags.

    $ cargo run

Currently there's no support for graphics, rather communication is achieved via the UART serial port.

I have both integration and unit tests using the custom test framework feature and cargo.
Unit tests are only run on the `lib` build.

    $ cargo test

## Debugging
For debugging purposes I use GDB built from the RISC-V GNU toolchain repo. 
[RISC-V GNU toolchain](https://github.com/riscv/riscv-gnu-toolchain). I believe one can use upstream GDB, but it may 
not be stable.

    $ git clone https://github.com/riscv/riscv-gnu-toolchain
    $ ./configure --prefix=$YOUR_PREFIX
    $  make build-gdb 
    # Add to PATH

Dependencies may need to be installed before the `./configure` step; instructions on what is 
required can be found in the repo README.

Once the debugger is built and on path, one can use the Makefile

    $ make debug

This launches a GDB session and a QEMU session in the background with open port localhost:1234 for GDB.

