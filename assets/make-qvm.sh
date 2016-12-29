#!/bin/sh

q3lcc -S $1.c
q3asm -v -m -vq3 -o $1.qvm $1.asm
