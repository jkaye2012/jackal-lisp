# Execution context layout

While Sahara can be describe as a [stack machine](https://en.wikipedia.org/wiki/Stack_machine), there is more to the
VM's construction than a single monolithic stack. It is important to understand the [memory and execution
model](./memory-execution-model.md) of the virtual machine in order to fully appreciate the details of the execution
context layout.

## Instruction memory

Instruction memory stores the bytecode instructions associated with each function/subroutine that is loaded into a
Sahara instance. A per-execution-context program counter points to the current location of the program within each
execution context. Debug and meta information may refer directly to instruction memory to provide their corresponding
functionality.

## Data stack

The data stack contains values that need not persist beyond the scope of a single stack frame. Instructions exist to
move data between the heap and the data stack, allowing compilers to define the lifetime of data as it moves through the
execution of a program.

## Call stack

The call stack contains the return addresses of functions as they're invoked within the VM and provides storage for
function-local variables. Values can be loaded to and from the data stack into the call stack using specialized
instructions.

## Local storage

For more information, see the [static memory](./static-memory.md) documentation.

## Heap

The heap stores data that must persist beyond the scope of a single stack frame. Unlike most other virtual machines, a
separate heap is allocated and managed per execution context, meaning that techniques like shared memory parallelism are
not possible within Sahara.

Data that must be shared across execution contexts must be sent explicitly using specialized messages meant for this
purpose. Data can either be moved or copied between heaps, but live references across heaps are never permitted. Within
a single context, any number of references can be made to the same data; because contexts are guaranteed to operate on a
single operating system thread, there is no risk of data races.

For more information see the [dynamic memory](./dynamic-memory.md) documentation.
