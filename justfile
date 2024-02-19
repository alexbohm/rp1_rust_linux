
rpi-copy:
    make V=1 KDIR=~/code/raspberrypi/linux ARCH=arm64 LLVM=1
    scp rust_out_of_tree.ko alex@rpi5.local:
