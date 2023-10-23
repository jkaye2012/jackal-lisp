# Bytecode specification

The Sahara bytecode aims to strike a balance between low-level operations for efficiency while maintaining basic
convenience for compilers. Sahara is a stack machine, so all instructions should be interpreted as operating relative to
a global instruction stack.

## Instruction layout

Each instruction provided by Sahara includes the following:

* A mandatory opcode
* One or more parameters
* An optional return value

Additionally, each instruction may consume 1 or more values from the top of the stack as arguments to the operation.
These are different from the parameters associated directly with the instruction in that stack arguments are generally
provided dynamically by preceding instructions while parameters are provided statically by compilers.

Opcodes are, for the time being, represented by a single byte. This means that at most 256 different instructions can be
supported. In actuality only 255 instructions should be supported before this is extended to either 1 or 2 bytes where
an initial 0xFF byte indicates an operation beyond 255.

Consider the `add` instruction as an example. The instruction can be defined as:

| Name | Opcode | Parameters | Stack            | Returns |
|------|--------|------------|------------------|---------|
| add  | 0      |            | numeric, numeric | numeric |

This tells us that the operation `add` has opcode `0`, takes no parameters, consumes the subsequent 2 values from the
top of that stack (both of which must be `numeric`), and pushes a `numeric` value to the top of the stack after
execution is complete.