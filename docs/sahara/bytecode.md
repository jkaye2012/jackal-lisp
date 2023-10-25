# Bytecode specification

The Sahara bytecode aims to strike a balance between low-level operations for efficiency while maintaining basic
convenience for compilers. Sahara is a stack machine, so all instructions should be interpreted as operating relative to
a global instruction stack.

## Instruction layout

Instructions are 32 bits wide and contain the following information:

* A mandatory 8-bit opcode
* Three optional 8-bit immediate values, `a`, `b`, and `c`

Each instruction is responsible for its own interpretation of the three immediate values, including whether they are
used at all.

Additionally, each instruction may consume 1 or more values from the top of the data stack as arguments to the
operation.  These are different from the parameters associated directly with the instruction in that stack arguments are
generally provided dynamically by preceding instructions while immediate arguments are provided statically by compilers.


## Example

Consider the `add` instruction as an example. The instruction can be defined as:

| Name   | Opcode | Parameters | Stack  | Returns |
|--------|--------|------------|--------|---------|
| add_u8 | 0      |            | u8, u8 | u8      |

This specifies that the operation `add_u8` has opcode `0`, does not interpret any immediate values, consumes the
subsequent 2 values from the top of that stack (both of which must be `u8`), and pushes a `u8` value to the top of the
stack after execution is complete.

## Specification