Self Hosted Compiler
====================

The task is to write a self hosted compiler, which means: A compiler written in the same language that it compiles. To make things way more easy, the target architecture is a theoretical CPU with minimal instruction set. There will be a simulator to test your programs in `_simulator`.

This Readme will give some information about the instruction set and the task itself.

## The Language
Turing complete. TODO: Finish section

## The CPU
The CPU is a 16 bit accumulator machine with a fairly limited instruction set. 16 bit means that the accumulator itself is 16 bit as well as a pointer is 16 bit wide. Every byte in memory can be addressed which means the address range spans `2^16` bytes.

### Your program
A program is given in either binary or (human readable) assembly. The compiler's output needs to be in binary format to solve this challenge.

To execute a program in binary format, you need to invoke the simulator as follows:
```
TODO
```
Your file is then loaded into the virtual memory, beginning at address 0. The PC is also set to 0, which means the first instruction that is executed is right in the beginning of your file.

### The instruction set
The instruction set was designed with the following goals: It shouldn't be too complex to learn very quickly. On the other hand it shouldn't lack important features that you'd need to simulate with other instructions.

The following is a summary of all instructions. `x` denotes an address (16 bit), `[x]` denotes "Data in memory at address `x`", "AC" means accumulator.

| opc | mnem | arg | effect                       | description                  |
| 00  | NOP  |     |                              | Does nothing                 |
| 01  | QUT  |     |                              | Quits execution              |
| 10  | LDA  | x   | AC <- [x]                    | Load [x] into AC             |
| 20  | STA  | x   | [x] <- AC                    | Store AC into [x]            |
| 30  | ADD  | x   | AC <- AC + [x]               | Adds [x] to AC               |
| 38  | COM  |     | AC <- ~AC                    | Ones complement, bitwise not |
| 39  | NEG  |     | AC <- ~AC + 1                | Twos complement              |
| 40  | AND  | x   | AC <- AC & [x]               | Bitwise AND                  |
| 90  | JMPI | x   | PC <- x                      | Unconditional jump to x      |
| 91  | JMP  | x   | PC <- [x]                    | Unconditional jump to [x]    |
| 92  | JZI  | x   | *if* AC = 0 *then* PC <- x   | Conditional jump to x        |
| 93  | JZI  | x   | *if* AC = 0 *then* PC <- [x] | Conditional jump to [x]      |
| C0  | OUTA |     |                              | Outputs AC                   |
