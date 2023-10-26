# Overview

Welcome to the documentation for the Jackal programming language. Jackal is an experimental language exploring some
opinionated ideas for language design and implementation. The language is currently in the research/development phase
and is not suitable for any serious use.

[Jackal](./jackal/index.md) is the programming language specification that developers can use to write their code. The
language is compiled to [custom bytecode](./sahara/bytecode.md) that can then either be interpreted or (in the future)
compiled directly to native machine code.

[Sahara](./sahara/index.md) is the virtual machine that provides the language runtime and most of Jackal's integrated
development and debugging functionality.

## Principles

Jackal's development is guided by opinionated principles. These are listed in their order of "importance"; that is,
earlier principles will be prioritized over later ones whenever a conflict should arise during language design and
development.

1. **Developer experience is paramount**
    * [Research suggests](https://queue.acm.org/detail.cfm?id=3595878) that developer experience is one of the most
      important factors in overall development productivity. Jackal's primary goal is to deliver a language, runtime,
      and ecosystem that developers love to use without sacrificing performance or correctness.

2. **A strong type system encourages correct software**
    * People often say of [Haskell](https://www.haskell.org/) "if it compiles, it probably works". This is possible only
      because of Haskell's usually strong and expressive type system. Jackal aims to provide similar guarantees through
      its type system, blending elements of functional and imperative paradigms to enable developers to encode their
      real-world constraints directly within the type system.

3. **Error handling is often more critical than the happy path**
    * In the absence of errors, software engineering is greatly simplified. The possibility of failure is often the
      source of the greatest complexity within a program. Because complexity is often inversely correlated with software
      quality, we must aim to minimize the overhead of recognizing and responding to errors within programs.

4. **Feedback loops must be fast and consistent**
    * Methodologies like [TDD](https://en.wikipedia.org/wiki/Test-driven_development) work for many people because they
      decrease the amount of time between a developer making a change and receiving feedback on whether their change had
      the intended effect. By providing interactive development environments that integrate the editing experience with
      a fully-functional execution environment, these loops can be shortened even further while also providing more
      functionality than is possible in most existing languages.

5. **Metaprogramming is underutilized in general and must be first-class**
    * Metaprogramming is an extremely useful tool that is often feared and underutilized. We believe this is mostly due
      to poor metaprogramming support in most languages. Jackal's implementation as a lisp allows it to include a fully
      hygienic macro system that integrates directly with the runtime features of the language. Proper use of
      metaprogramming techniques greatly reduces the total amount of code required to solve most problems, and less code
      will mean less defects and less maintenance for most programs.

6. **Performance cannot be an afterthought**
    * Good performance is a necessity for functional software. While the primary goal o Jackal is to improve developer
      experience and program correctness, the impact of these decisions on the overall performance of the final output
      should always be considered. While performance matching that of purely native code is unlikely to be possible,
      relative comparison to the performance of optimized C programs will be our guide.

## Code examples

## Next steps
