# Dynamic memory

Sahara supports two different forms of dynamic memory:

* A dedicated heap with fully dynamic extent for each [execution context](./execution-context.md)
* A shared global heap subject to significant restrictions

Memory is allocated and managed differently depending upon where it's allocated and how its ownership is specified.

## Execution context heaps

Each execution context has a heap allocated for its exclusive use when the context is created. Properties of the heap
like minimum size, maximum size, or whether the heap is dynamically or statically sized can be controlled by the user
upon context creation.

By default, data allocated within an execution context is stored on that context's exclusive heap. Because this heap is
guaranteed to be accessed by only a single execution context, Sahara is able to rely on the fact that parallel access to
this heap is impossible by managing the data's lifetime using only its ownership.

### Ownership

Three different modes of ownership are supported:

* `Unique` ownership means that the lifetime of the data is managed by a single unique owner. This should be by far the
  most common ownership mode in well-constructed programs
* `Shared` ownership means that the lifetime of the data is managed by two or more owners, including the possibility of
  a dynamic number of owners. This ownership mode should be rare, as multiple ownership is not common in
  well-constructed programs. Due to its dynamic nature, use of shared ownership requires runtime reference counting
  (each time an owner is added or removed from the shared memory) resulting in a small performance impact to this
  ownership mode
* `Reference` ownership means that an owner needs access to the shared memory, but should not participate in the
  memory's lifetime. This means that the memory's existence must be validated and maintained atomically each time that
  the memory is accessed. Most of the time that shared ownership is being considered, unique ownership with multiple
  reference owners is likely to be the better option

Memory is automatically freed based on the (recursive) lifetime of its owner(s):

* `Unique` memory is freed when its owner is freed
* `Shared` memory is freed when _all_ of its shared owners are freed
* `Reference` memory has no effect on when memory is freed as it does not participate in ownership semantics

## The global heap

The global heap allows memory to be shared between execution contexts without explicit message passing. Because this
violates the shared-nothing foundations of execution contexts, this heap is subject to strict requirements and has worse
runtime performance characteristics than each context's dedicated heap. It is not a general purpose heap in the
traditional sense. Therefore, usage of the global heap should be restricted to situations only when either:

* There is no other way to accomplish a goal
* Use of the global heap significantly simplifies an implementation, and the performance impact is acceptable
* Working with read-only data that can be populated fully upon creation

### Allocation modes

When memory is allocated on the global heap, it must be allocated in one of three possible modes:

* `Constant`: memory is populated when allocated, never modified, and never freed
* `Static`: memory is allocated and never freed, but may be modified
* `Dynamic`: memory is allocated and may be both modified and freed

The modes are listed in order from most performant to least. `Constant` memory requires no ownership tracking or
synchronization, so has no performance overhead. `Static` memory requires no ownership tracking, but concurrent access
to the memory must be protected against. `Dynamic` memory requires both ownership tracking and concurrent access
protection.