# rustyriscy
A Rust OS targeting Risc-V

# Dependencies
## Toolchain
The following sets up the correct toolchain and target for builds, this assumes using rustup:

    rustup default nightly
    rustup target add riscv64gc-unknown-none-elf
    cargo install cargo-binutils
