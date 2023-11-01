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
operation. These are different from the parameters associated directly with the instruction in that stack arguments are
generally provided dynamically by preceding instructions while immediate arguments are provided statically by compilers.

## Concepts

To fully understand the instructions documented here, you must have at least a passing understanding of the [execution
context layout](./execution-context.md). Specifically, many instructions make reference to:

* The [data stack](./execution-context.md#data-stack)
* The [constant pool](./execution-context.md#constant-pool)
* The [local storage](./execution-context.md#local-storage)
* The [heap](./execution-context.md#heap)

The description of these instructions are unlikely to be interepretable without an understanding of how each of these
regions is laid out and how they are meant to be interacted with.

## Values

A limited number of primitive types are supported by all instructions. Sahara calls these its Values. More documentation
on the supported types and their formats can be found in the [Value documentation](./value.md). In the bytecode
specification that follows, there are a number of different Value subsets that are used in various instructions:

* Individual value types may be specified when they are required directly
    * E.g. `char` or `u64`
* `value` refers to any type supported by Value
* `numeric` refers to any of the numeric types supported by Value
    * `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`

## Example

Consider the `add` instruction as an example. The instruction can be defined as:

| Name   | Opcode | Parameters | Stack  | Returns |
|--------|--------|------------|--------|---------|
| add_u8 | 0      |            | u8, u8 | u8      |

This specifies that the operation `add_u8` has opcode `0`, does not interpret any immediate values, consumes the
subsequent 2 values from the top of that stack (both of which must be `u8`), and pushes a `u8` value to the top of the
stack after execution is complete.

Instructions that utilize immediate parameters will look like:

| Name    | Opcode | Parameters | Stack | Returns |
|---------|--------|------------|-------|---------|
| const   | 254    | abc: cidx  |       | value   |
| imm_u8  | 251    | a: u8      |       | value   |
| imm_u16 | 250    | ab: u16    |       | value   |

For these instructions, no data in consumed from the data stack. Instead, information is provided via the instruction
directly using its immediate bits. The `imm_u8` instruction must set the 8 `a` bits to specify the `u8` that should be
pushed on to the data stack. The `imm_u16` instruction must set the 8 `a` bits and 8 `b`b bits to specify the `u16` that
should be pushed onto the data stack. Finally, the `const` instruction must set all 24 immediate bits (`a`, `b`, and
`c`) to specify the constant index (`cidx`) that the VM can use to load the wider constant value that will then be
pushed onto the data stack.

`ab` means that the `a` and `b` bytes will be interpreted as a singular value by the instruction. Instructions that
interpret these bytes separately would instead be specified as `a: u8, b: u8`.

## Immediate types

Most of the immediate parameter types within the specification are obvious. A few are specific to Sahara that could be
confusing:

* `cidx`: an index into the global constant pool
* `fidx`: an index into the global function table
* `hidx`: a memory location in the current execution context's heap
* `lidx`: a relative offset into the current execution context's local storage

Each of these should be read as a mnemonic; they are pronounced "constant index", "function index", "heap index", and
"local index" respectively.

## Specification

### Loading constant values

There are a number of instructions provided to load constants directly onto the data stack. Each instruction loads a
specific type of value. There are two flavors of value loading depending upon the bit width of the value being loaded.
Values with a width of 24 bit or smaller are loaded using the instruction's immediate bits, while those with a width of
25 bits or greater are loaded by index into the constant pool. For instructions that load from the constant pool. the
immediate bits _must_ refer to an entry within the constant pool containing the correct type; otherwise, the VM will
panic.

| Name     | Opcode | Parameters | Stack | Returns | Description                                                    |
|----------|--------|------------|-------|---------|----------------------------------------------------------------|
| const    | 254    | abc: cidx  |       | value   | Load a constant from the constant pool onto the data stack     |
| imm_bool | 253    | a: bool    |       | bool    | Load a boolean immediately onto the data stack                 |
| imm_char | 252    | a: u8      |       | char    | Load a character immediately onto the data stack               |
| imm_u8   | 251    | a: u8      |       | u8      | Load an 8 bit unsigned integer immediately onto the data stack |
| imm_u16  | 250    | ab: u16    |       | u16     | Load a 16 bit unsigned integer immediately onto the data stack |
| imm_i8   | 249    | a: i8      |       | i8      | Load an 8 bit signed integer immediately onto the data stack   |
| imm_i16  | 248    | ab: i16    |       | i16     | Load a 16 bit signed integer immediately onto the data stack   |

### Function invocation

Functions can be invoked after they are registered in the [function table](./functions.md#function-table).
Invocation is performed by referencing the desired [function index](./functions.md#function-indices).

| Name   | Opcode | Parameters | Stack | Returns | Description                                                          |
|--------|--------|------------|-------|---------|----------------------------------------------------------------------|
| call   | 6      | abc: fidx  |       |         | Invoke the function referred to by the immediate function index      |
| return | 7      |            |       |         | Return from the current function, moving one level up the call stack |

### Function local variables

Variables local to a function are implemented with register-like functionality. When new locals are added to a function,
they are indexed using the order in which they are defined (beginning with index 0). Locals can then be read from their
"register" onto the data stack or reassigned. Deallocation of locals is not currently supported. When a function exits
(via the `return` instruction), all locals associated with the function are automatically and permanently deallocated.

| Name        | Opcode | Parameters | Stack | Returns | Description                                             |
|-------------|--------|------------|-------|---------|---------------------------------------------------------|
| local_store | 8      |            | value |         | Store a value into a local variable from the data stack |
| local_read  | 9      | abc: lidx  |       | value   | Load a local variable onto the data stack               |

### Arithmetic operations

Most arithmetic operations are "polymorphic" in that they accept `Values` of any numeric type, allowing promotion
between compatible types. For this promotion to function properly, the larger of the two types must be placed on the top
of the data stack. For example, to successfully add a `u8` to a `u32`, the `u32` value must be placed on top of the
stack with the `u8` value directly following. If a smaller value is placed on the top of the stack, the operation will
panic. For non-commutative operations, this means that operands will have to be explicitly converted if their sizes are
different. While this may be inconvenient for the user, any other implementation cannot guarantee correctness.

| Name | Opcode | Parameters | Stack            | Returns | Description                                     |
|------|--------|------------|------------------|---------|-------------------------------------------------|
| add  | 1      |            | numeric, numeric | numeric | Add the two values on the top of the stack      |
| sub  | 2      |            | numeric, numeric | numeric | Subtract the two values on the top of the stack |
| mul  | 3      |            | numeric, numeric | numeric | Multiply the two values on the top of the stack |
| div  | 4      |            | numeric, numeric | numeric | Divide the two values on the top of the stack   |

### Interacting with data types

| Name | Opcode | Parameters | Stack | Returns | Description |
|------|--------|------------|-------|---------|-------------|
|      |        |            |       |         |             |

### IO operations

IO operations allow Sahara to interact with the "external" world.

| Name  | Opcode | Parameters | Stack | Returns | Description                                             |
|-------|--------|------------|-------|---------|---------------------------------------------------------|
| print | 5      |            | value |         | Output the value on the top of the data stack to stdout |
