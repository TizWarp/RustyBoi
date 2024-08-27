# Rusty Boi
### Rusty Boi is my Game Boy inspired Language VM written in Rust. 


## Overview

- RAM : 65kb 
- Program Memory : 65kb
- Eight 16bit registers that can be used as two 8bit registers
- Flag Register for carries (unused) and comparisons

---

The VM will read instruction from the Program memory only

### Memory Map
0x0000 -> Up Input Address -> Up arrow/W
0x0001 -> Down Input Address -> Down Arrow/S
0x0002 -> Right Input Address -> Right Arrow/D
0x0003 -> Left Input Address -> Left Arrow/A

0x0004 -> Workable Ram Start
0x87FE -> Stack Start / Workable Ram End

0x87FF -> Graphics Ram Start
0xFFFF -> Graphics Ram End

### Display
120x80 24 bit RGB display
Each tile is assigned 3 memory bytes starting at 0x87FF
The first byte is the tiles red, the second blue and third green

The screen is updated whenever the VM returns from it's internal run loop. this will happen when the PC reaches the end of the Program Memory or when certain Opcodes are ran


## Op Codes

#### Rusty Boi follows a RISC architecture

| Name |  Operands | Function |
|--|--| --|
| LOAD | Register Immediate | Loads an immediate into the specified register, will automatically switch between 8bit and 16bit based on supplied register.
|MOV|Register Register | Copies the value from the first register into the second register, supplied registers must be the same bit width
|STORE|Register|Copies the value from the register into the memory address currently loaded in register 7
|READ|Register|Copies the value from the memory address in register 7 into the supplied register
|PUSH|Register|Pushes the value in the supplied register onto the stack
|POP|Register|Pops the a value off the stack and copies it into the supplied Register
|EQ| Register Register| Sets the compare bit to one if values in supplied registers are equal
|NEQ|Register Register| Opposite of EQ|
|LES| Register Register | Sets flag bit to one if register ones value is less than the seconds registers value. Sets flag to 0 if false. Treats register values as unsigned
|LESi|Register Register | Like LES but will treat values as signed|
|ADD|Register Register Register | Adds the values from the first two registers and puts the result into the third register|
|ADDi| Register Register Register| Signed ADD
|SUB| Register Register Register| Subtracts the seconds register from the first and puts the result into the third register|
|SUBi|Register Register Register|Signed SUB|
|MUL| Register Register Register | Multiplies the first two register together and puts the result into the third register|
|MULi| Register Register Register|Signed MUL|
|DIV| Register Register Register |Divides the first register by the second, and puts quotient into the third|
|DIVi| Register Register Register | Signed DIV|
|MOD| Register Register Register | Mods the first register by the second and puts the result in the third|
|MODi| Register Register Register | Signed MOD|
|SHR| Register Register Register | Shifts the first registers value right a number of bits equal the seconds Register, puts the result in the third Register
 |SHL| Register Register Register | Shifts the first registers value left a number of bits equal the seconds Register, puts the result in the third Register
 |JMP| 16bit Immediate | Jumps the PC to the supplied 16 bit address|
 |CJMP| 16bit Immediate|Jumps the PC to the supplied 16 bit address when the compare bit is set to 1|
 |NJMP| 16bit Immediate | Jumps the PC to the supplied 16 bit address when the compare bit is set to 0
 |CALL| 16bit Immediate | Pushes the next Opcode Address onto the return stack and then jumps the supplied 16bit address
 |CCALL| 16bit Immediate| CALL when compare flag is 1|
  |NCALL| 16bit Immediate| CALL when compare flag is 0|
  |RET| N/A | Jumps the PC to the most recent return stack address|
  |DRAW|N/A| Causes the VM to redraw the window and resume at the next opcode|
  |RDRAW|N/A|Causes the VM to redraw and set the PC to 0
  |JDRAW|16bit Immediate|Causes a redraw then sets the PC to supplied 16bit Immediate|
  
### Assembly Syntax

Opcode are written as above, operands are space separated.
A 16 register is declared by $ symbol followed by a number 0 - 7
To select a 8bit half of a register add an "a" for the first half or a "b" for the second half
All opcodes that take register operands will automatically select the correct width.
All register operands for an opcode must share the same width
Constants are declared as follows : "const "name" "immediate value"
Immediate values are either a decimal prefixed with a "#" or hex prefixed with a "0x"
Labels declarations are a name followed by a colon "name:" when referencing a label do not include the colon

#### Syntax Examples 
```
main:

CALL clear_screen

JDRAW main

clear_screen:
LOAD $7 0x87FF
LOAD $6 0x87FF
LOAD $5 #1
LOAD $0a #255

loop:
STORE $0a
ADD $7 $5 $7

LES $6 $7
CJMP loop

RET
```

## App Usage

Compiling
cargo run c [input_file] [output_file]
Executing
cargo run e [program_file] 
