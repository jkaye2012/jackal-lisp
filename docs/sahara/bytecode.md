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
* `heap` refers to a dynamic heap allocation (a `u64` index into the context heap)
* `ref` refers to a non-owning reference to a dynamic heap allocation

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

* `cidx`: `constant index`, an index into the global constant pool
* `fidx`: `function index`, an index into the global function table
* `tidx`: `type index`, an index into the global type definition table
* `lidx`: `local index`, a relative offset into the current execution context's local storage
* `didx`: `data index`, a relative offset into a data type's [field definition](./data-types.md#type-definitions)

Each of these indices is 24 bits wide, occupying the `abc` bits using big endian encoding.

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

### Instruction extension

There are situations where 32 bits are not enough to encode all information required by an instruction. In these cases,
the `extend` instruction can be used up to 7 times to "extend" an instruction with additional information. Extend
instructions should immediately precede the instruction that they are meant to extend.

| Name   | Opcode | Parameters | Stack | Returns | Description                                                           |
|--------|--------|------------|-------|---------|-----------------------------------------------------------------------|
| extend | 247    | arbitrary  |       |         | Extend a subsequent instruction with additional immediate information |

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
| local_store | 8      | abc; lidx  | value |         | Store a value into a local variable from the data stack |
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

| Name          | Opcode | Parameters                   | Stack           | Returns | Description                                                 |
|---------------|--------|------------------------------|-----------------|---------|-------------------------------------------------------------|
| dt_create     | 10     | abc: tidx                    | multiple values | value   | Create a [type instance](./data-types.md#type-instances)    |
| dt_read_field | 11     | abc: lidx, ext: field offset |                 | value   | Load the value of a data type's field onto the data stack   |
| dt_set_field  | 12     | abc: lidx, ext: field offset | value           | value   | Update the value of a data type's field from the data stack |

#### dt_create

The `dt_create` instruction warrants a more in-depth description. The instruction expects an extension so that it can
accepts a single immediate parameter: a local slot index. The type definition (extracted from the local slot metadata)
contains information about fields required to create an instance of the type; namely, the number of fields and the type
of each field.

Callers should ensure that a single value for each required field is pushed onto the data stack before `dt_create` is
invoked. The instruction will pop values off of the stack for each required field, expecting that fields have been
pushed in reverse order relative to the type definition.

For example, consider the data type:

```lisp
(data Rgb
 [red u8]
 [green u8]
 [blue u8])
```

The type definition entry for this type would look like:

```yaml
name: Rgb
fields:
  - name: red
    type: u8
  - name: green
    type: u8
  - name: blue
    type: u8
```

To create the color cyan (`0x00FFFF`), the user would push the `blue` byte, then `green`, then `red`, followed by the
create instruction. Assuming that the type definition for `Rgb` should be stored in local slot `4`, this creation would
be represented by:

```
imm_u8 0xFF
imm_u8 0xFF
imm_u8 0x00
dt_create 0x4
```

#### dt_read_field and dt_set_field

Reading and setting data type fields requires use of the [extend instruction](#instruction-extension) to provide two
immediate parameters: the local slot index holding the data, and the field offset (relative to the local slot)
containing the field that should be read or updated. The field offset can be retrieved from the type definition
registered with the VM.

Continuing with our previous example, to read the `green` field from the data that we created at slot index `4`, the
user would issue the following instructions:

```
extend 0x1
dt_read_field 0x4
```

This reads the third field (zero indexed) from the local in slot `4`, which is the value of `green`.

### IO operations

IO operations allow Sahara to interact with the "external" world.

| Name  | Opcode | Parameters | Stack | Returns | Description                                             |
|-------|--------|------------|-------|---------|---------------------------------------------------------|
| print | 5      |            | value |         | Output the value on the top of the data stack to stdout |

### Dynamic memory management

Automatic memory management is implemented using a simple reference counting mechanism. When an allocation is created, it must be stored in a stack local;
this local slot is its sole referrer, and thus the allocation's reference count is initialized to 1.

Reference counts are incremented whenever:

1. An allocation is stored into a stack local
2. An allocation is stored into a field of another allocation

Reference counts are decremented whenever:

1. A stack frame is popped containing one or more references to an allocation in its local slots
2. An allocation is freed containing one or more references to an allocation in its fields

Either of these situations can cause multiple decrements to occur at once.

When an allocation's reference count reaches 0, its memory is freed and any other allocations that it refers to have their reference counts decremented as
described above.

### Circular references

Currently, automatic memory management does not detect circular references and thus will leak memory unless one of the allocations is manually
freed.

| Name       | Opcode | Parameters        | Stack           | Returns | Description                             |
|------------|--------|-------------------|-----------------|---------|-----------------------------------------|
| heap_alloc | 13     | abc: lidx         | multiple values | heap    | Dynamically allocate memory for a type  |
| heap_store | 14     | abc: field offset | heap, value     | value   | Store a value into a dynamic allocation |
| heap_read  | 15     | abc: field offset | heap            | value   | Reads a value from a dynamic allocation |

#### `heap_alloc`

`heap_alloc` is the basic heap allocation function. The data stack should contain similar values as `dt_create` (one value
for each field in the type referred to by `abc`). Allocated data is owned by the stack frame in which the allocation is
performed; if the memory is not moved or shared, it will be deallocated when the stack frame is popped.

Allocations created in this way must always be stored in a stack local. An [extended
instruction](#instruction-extension) should be used to specify the local slot to which the allocation should be stored.

#### `heap_store`

`heap_store` places a value into a field within a heap allocation. The expects two values to exist
on the data stack: the owning heap allocation and the value to store. The immediate parameter is the field index
into the owning allocation that should be set with the value. Any currently owned allocation in the field
index may be deallocated if there are no other active owners. The stored value is left on the stack following the
operation.

#### `heap_read`

`heap_read` reads a value from a field within a dynamic allocation. The allocation to read from should be on the data
stack, while the immediate parameter denotes the field index to be read.
