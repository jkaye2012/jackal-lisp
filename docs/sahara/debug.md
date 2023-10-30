# Debug information

Debug information provides Sahara with an understanding of the source code that was used to generate a bytecode program
run within the virtual machine. Debug information is always optional and in no way affects the live execution of a
program outside of the Sahara debugger.

While debug information maps VM instructions to source code, [meta information](./metaprogramming.md) provides
additional VM-level information about each function/value loaded into a Sahara program. As such, the debugger often
relies upon meta information for some of its more advanced functionality.

A useful way to think about the distinction between meta information and debug information is that debug information is
provided to Sahara by a source language compiler, while meta information is created and tracked automatically within
Sahara itself as it analyzes a program that is loaded into the virtual machine. Meta information can also be useful
during live execution for semantic metaprogramming operations, while debug information is useful only within an
interactive debugging session.

## Format

Debug information allows for specification of source code information for any of the following constructs:

* [Modules](./modules.md)
* [Functions](./functions.md)
* [Instructions](./bytecode.md)
* [Values](./value.md) (including the data stack, constant pool, and local variables)

### Modules

### Functions

### Instructions

### Values