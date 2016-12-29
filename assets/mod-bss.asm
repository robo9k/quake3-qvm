export vmMain
code
proc vmMain 0 0
CNSTI4 -1
RETI4
LABELV $1
endproc vmMain 0 0
bss
export uninitialized
align 4
LABELV uninitialized
skip 4
