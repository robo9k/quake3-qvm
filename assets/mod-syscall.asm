export vmMain
code
proc vmMain 0 4
ADDRGP4 $2
ARGP4
ADDRGP4 trap_Print
CALLI4
pop
CNSTI4 -1
RETI4
LABELV $1
endproc vmMain 0 4
import trap_Print
lit
align 1
LABELV $2
byte 1 72
byte 1 101
byte 1 108
byte 1 108
byte 1 111
byte 1 44
byte 1 32
byte 1 119
byte 1 111
byte 1 114
byte 1 108
byte 1 100
byte 1 33
byte 1 0
