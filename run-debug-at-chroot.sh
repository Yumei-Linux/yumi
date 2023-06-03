#!/usr/bin/env bash

# requires yumei-chroot, cargo, xclip and rust in this machine
# you also have to mount your yumei-root disk at /mnt
# or change the path of it in the config variables of this shell script.

# > remember to run this script as root.

ROOT=/mnt
PREFIX=/usr

# -----

cargo build

test -f $ROOT/$PREFIX/bin/yumi &&
    rm -rvf $ROOT/$PREFIX/bin/yumi

install -Dvm755 ./target/debug/yumi $ROOT/$PREFIX/bin/yumi

# -----

echo "* Done, now paste the code i've copied in your clipboard"
echo "yumi ${@}" | xclip -sel c

yumei-chroot $ROOT