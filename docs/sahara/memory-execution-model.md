# Memory and execution model

Sahara's memory allocation and execution model are somewhat restrictive and opinionated. In return, the runtime is
able to guarantee memory safety, automatically manage memory, and prevent race conditions entirely without requiring
complex reference management systems in source languages.

## Execution contexts

The primary concept is the _execution context_. An execution context is an independent [green
thread](https://en.wikipedia.org/wiki/Green_thread) that is guaranteed to operate on a single operating system thread.
Instructions never span execution contexts, and data can cross between execution contexts only with explicit operations
that either copy the data or transfer its ownership to the receiving context entirely. Unlike most other execution
environments, this means that Sahara supports neither shared-memory parallelism nor an unconstrained program-wide shared
heap.


An execution context can be visualized like so:

![Execution context](../images/ExecutionContext.png)

More information about each element within the execution context can be found in the [execution context
documentation](./execution-context.md).

## Context allocation

## Program execution

## Concurrency

## Automatic memory management

Memory is automatically managed differently depending upon where the memory was allocated and how it is owned. All
dynamically allocated data in Sahara must be annotated with its corresponding ownership semantic when it is created.
More information on the supported heaps and ownership modes can be found in the [dynamic memory](./dynamic-memory.md)
documentation.

## Scheduling