#!/bin/sh

q3lcc -S mod.c
q3asm -m -vq3 -o mod.qvm mod.asm
