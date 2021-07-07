#!/bin/bash

wget https://mirrors.opensuse.org/ -O /dev/shm/mirror.html
cat /dev/shm/mirror.html | ./target/debug/get_mirror_data $1
