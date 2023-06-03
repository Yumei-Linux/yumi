#!/usr/bin/env bash

# requires yumei-chroot, git in this machine
# you also have to mount your yumei-root disk at /mnt
# or change the path of it in the config variables of this shell script.

# > remember to run this script as root.

ROOT=/mnt

[ -d $ROOT/var/yumi ] && rm -rf $ROOT/var/yumi
git clone https://github.com/Yumei-Linux/yumi-packages.git $ROOT/var/yumi