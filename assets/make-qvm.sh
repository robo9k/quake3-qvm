#!/bin/sh

q3lcc -S $1.c
q3asm -m -vq3 -o $1.qvm $1.asm
