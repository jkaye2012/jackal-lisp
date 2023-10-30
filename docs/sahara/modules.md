# Modules

Modules are a mechanism for code isolation, packaging, and distribution. Modules as a first-class concept within the
Sahara virtual machine means that a single program could theoretically load code from multiple source languages
simultaneously. This also means that a library written in any source language that targets Sahara can be loaded as a
dependency of any other source language. This is possible because each module is treated independently and is
interpreted in a consistent format regardless of the original source.

## Scoped names

Modules introduce the concept of the **scoped name**. Scoped names satisfy the regular expression:

```regex
[a-z][a-z\-]*(::[a-z][a-z\-]*)*
```

Examples of scoped names include:

* `std`
* `std::map`
* `awesome::module::name`

## Contents

A module consists of the following:

* Required:
    * A scoped name
* Any number of optional objects:
    * [Functions](./functions.md)
    * [Data types](./data-types.md)
    * [Traits or implementations](./traits.md)
    * [Effects or interpreters](./effects.md)
    * [Conditions or handlers](./conditions.md)

## Loading objects from modules

All VM instructions that load objects do so in the form of their **fully-qualified name**, or `fq_name` for short. This
means that in order to access an object provided by a module, compilers must resolve objects relative to their
corresponding module. Sahara has no concept of a global namespace, meaning that objects must always be resolved from a
module.

Perhaps unsurprisingly, module names must be globally unique within the context of a single program. If instantiation of
the same module is attempted multiple times, the VM will panic.

## Data layout

Modules are conceptual within the VM - they do not have a physical layout. Instead, each type of object is stored
independently using its fully-qualified name. As objects are being loaded into the VM, they are loaded along with their
registered module, providing the fully qualified name of each object.
