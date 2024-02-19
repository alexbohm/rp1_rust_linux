# Rust out-of-tree module

This is a basic template for an out-of-tree Linux kernel module written in Rust.

Please note that:

  - The Rust support is experimental.

  - The kernel that the module is built against needs to be Rust-enabled (`CONFIG_RUST=y`).

  - The kernel tree (`KDIR`) requires the Rust metadata to be available. These are generated during the kernel build, but may not be available for installed/distributed kernels (the scripts that install/distribute kernel headers etc. for the different package systems and Linux distributions are not updated to take into account Rust support yet).

  - All Rust symbols are `EXPORT_SYMBOL_GPL`.

Example:

```sh
$ make KDIR=.../linux-with-rust-support LLVM=1
make -C .../linux-with-rust-support M=$PWD
make[1]: Entering directory '.../linux-with-rust-support'
  RUSTC [M] .../rust-out-of-tree-module/rust_out_of_tree.o
  MODPOST .../rust-out-of-tree-module/Module.symvers
  CC [M]  .../rust-out-of-tree-module/rust_out_of_tree.mod.o
  LD [M]  .../rust-out-of-tree-module/rust_out_of_tree.ko
make[1]: Leaving directory '.../linux-with-rust-support'
```

```txt
[    1.076945] rust_out_of_tree: Rust out-of-tree sample (init)
[    1.084944] rust_out_of_tree: My numbers are [72, 108, 200]
[    1.085944] rust_out_of_tree: Rust out-of-tree sample (exit)
```

For details about the Rust support, see https://rust-for-linux.com.

For details about out-of-tree modules, see https://docs.kernel.org/kbuild/modules.html.

## rust-analyzer

Rust for Linux (with https://lore.kernel.org/rust-for-linux/20230121052507.885734-1-varmavinaym@gmail.com/ applied) supports building a `rust-project.json` configuration for [`rust-analyzer`](https://rust-analyzer.github.io/), including for out-of-tree modules:

```sh
make -C .../linux-with-rust-support M=$PWD rust-analyzer
```

# RP1 Rust Driver
This repo uses the basic template from the Rust for Linux project:
https://github.com/Rust-for-Linux/rust-out-of-tree-module

## Getting Rust on the Raspberry Pi 5 ##
In order to get Rust on the raspberry pi 5, we need to compile a custom kernel.

This guide from raspberry pi is very useful, I follow many of the same exact steps but with some tweaks for LLVM:
https://www.raspberrypi.com/documentation/computers/linux_kernel.html

* I used kernel version 6.8 on the raspberry pi fork.
  * https://github.com/raspberrypi/linux/tree/rpi-6.8.y
* Apply patch from Rust for Linux branch rust-pci to the pi kernel.
  * https://github.com/Rust-for-Linux/linux/tree/rust-pci
  * https://github.com/torvalds/linux/compare/master...Rust-for-Linux:linux:rust-pci.patch
* Run the kernel defconfig from the pi kernel guide, but with some tweaks:
  * `make ARCH=arm64 LLVM=1 bcm2712_defconfig`
  * Note the change from CROSS_COMPILE to LLVM because Rust uses the LLVM toolchains.
* Enable Rust support for arm64
  * Use the quick start guide to get depedencies set up: https://docs.kernel.org/rust/quick-start.html
  * Add `select HAVE_RUST` to arch/arm64/Kconfig
  * Edit `scripts/generate_rust_target.rs` so that it supports ARM64. I think I looked at what Asahi used for these options.
  * Use menuconfig to enable Rust support.
    * General Setup -> Rust support
    * Press z to show hidden options, then viewing the help for Rust support will show you the depends which are inhibiting the option.
    * I had to turn off MODVERSIONS to satisfy the depends for Rust support.
* Add ioremap to the rust helpers.c.
  * I think the ioremap supplied by CONFIG_IOREMAP_GENERIC doesn't let bindgen generate ioremap automatically so the helper is needed.
* Compile the kernel
  * See the linux kernel guide from raspberry pi above. The kernel compile and install to the sd card is very similar, but with LLVM instead of CROSS_COMPILE. These are the commands that are different:
  * `make ARCH=arm64 LLVM=1 Image modules dtbs -j20`
  * `sudo env PATH=$PATH make ARCH=arm64 LLVM=1 INSTALL_MOD_PATH=sdcard/ext4 modules_install`
