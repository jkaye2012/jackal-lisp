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

## Return stack

The return stack contains the addresses of functions as they're invoked within the VM. This allows execution to easily
continue at the appropriate point upon completion of function execution.

## Heap

The heap stores data that must persist beyond the scope of a single stack frame. Unlike most other virtual machines, a
separate heap is allocated and managed per execution context, meaning that techniques like shared memory concurrency are
not possible within Sahara.

Data that must be shared across execution contexts must be sent explicitly using specialized messages meant for this
purpose. Data can either be moved or copied between heaps, but live references across heaps are never permitted. Within
a single context, any number of references can be made to the same data; because contexts are guaranteed to operate on a
single operating system thread, there is no risk of data races.

## Meta information

Meta information is required metadata that Sahara programs and development tooling can use to inspect the structure of a
program at compile time. Object models, function signatures, and condition details are all examples of the type of
meta information that may be of interest to developers.

## Debug information

Debug information is optional metadata that development tooling can use to inspect the state of a Sahara program at
run time. Debug information is not loaded by default. Debug information allows programs to be paused, queried, and even
modified while they're running. It also allows the Sahara VM to automatically provide useful information like code
coverage when instrumentation is enabled.

There is also a runtime cost associated with debug information in order to provide dynamic debugging functionality, such
as the ability to refer to objects on the heap and the data stack by their corresponding names in the source code.



