# The Sahara Virtual Machine

Sahara is a stack-based virtual machine tailored to the run-time and development introspection capabilities required by
the Jackal programming language. While the VM supports only a single bytecode representation, there are ancillary debug
and metaprogramming information formats that compilers can generate to provide functionality beyond the standard runtime
execution of the generated program.

## Why not registers?

Register machines, while potentially faster than stack machines, are much more complicated to implement and reason about
than stack machines. Further, once AOT/JIT compilation is introduced, the performance differences between register and
stack machines diminishes significantly. It's unlikely that the incremental performance benefit that could be provided
by a register machine implementation is worth the additional complexity considering that JIT compilation is likely to be
required for adequate performance in either case.

Finally, a stack machine (with sufficient custom extensions and operations) lends itself to better dynamic introspection
and manipulation through Jackal's integrated development tooling than a register machine. Manipulating the values or
operations of a single register could require cascading modifications to the rest of the register allocation algorithm,
greatly increasing the complexity of implementing such behavior without requiring recompilation of entire units.

## Specifications

* The [bytecode specification](./bytecode.md) defines the provided runtime operations
* The [debug info specification](./debug.md) defines the provided debugging operations
* The [metaprogramming specification](./metaprogramming.md) defines the provided metaprogramming operations