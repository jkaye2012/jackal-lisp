# Data layout

As specified in the [execution context documentation](./execution-context.md), Sahara operates using a few different
concepts to provide its runtime behavior:

* Instruction memory
* Data stack
* Call stack
* Heap

Each of these elements has a unique data layout. Interactions between the various elements are defined in terms of their
differing layouts. This includes the instruction set architecture, which has been designed with these layouts in mind.

## Instruction memory

Instruction memory is stored as a contiguous block of [bytecode instructions](./bytecode.md). Instructions are
fixed-size with a width of 32 bits. Each instruction within a program can be referred to by its _instruction address_,
which can be thought of as an index into the program pointing at the individual instruction within instruction memory.

## Values

Values are Sahara's natively supported types. Instructions read/write Values to/from the data stack and call stack.

## Data stack

The data stack is where instructions read and write data to perform virtual machine operations. The data stack is stored
as a contiguous block of Values. The data stack should be used only for values that must cross function boundaries,
whereas the call stack should be used for local value storage.

## Call stack

The call stack holds information about the currently executing function, including the instruction address that should
be returned to after function execution is complete and any local values required by the function.

## Heap

The heap holds data with either dynamic size or dynamic lifetime.