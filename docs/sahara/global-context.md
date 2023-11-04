# Global context layout

Sahara stores read-only data that can be used by all [execution contexts](./execution-context.md) concurrently in its
_global context_. The global context is constructed during compile time and is used almost exclusively for performance
optimization and instruction simplicity.

## Constant pool

## Function definitions

## Type definitions

---

These were moved from their original home in the execution context documentation. It's likely that these concepts are
going to evolve as the implementation of the VM progresses. It's possible that they may not end up existing at all in
the form documented here; however, if they do end up in any similar form, the concepts will be represented globally
rather than per execution context, making this the more correct location for this documentation.

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


