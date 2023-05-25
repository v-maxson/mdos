# MDOS
OS Developement Research

> Note: Subprojects also contain `README` files. They probably won't contain much information for a while.

### Requirements
- [Rust](https://www.rust-lang.org) 1.69.0+
    - `cargo-make`
    - `x86_64-unknown-none` target
- [QEMU](https://www.qemu.org)
    - Ensure `qemu` installation folder is added to `PATH`.
    - Optional, only required for the `cargo make run-qemu` task.

### Building
- `cargo make gen-img`
  - Generates an `out` directory containing the generated image.
- `cargo make run-qemu`
  - Runs `cargo make gen-img`, then launches QEMU with generated image.