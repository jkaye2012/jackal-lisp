# Data Types

Sahara provides support for compound data types. Data types are a complex [value](./value.md) that may implement functionality in a number of ways:

* **Fields** are named [values](./value.md) associated with a data type
    * Note that this implies the recursive nature of data types
* **Methods** are [functions](./functions.md) associated with a data type
* **Implementations** associate [traits](./traits.md) with a data type

Additionally, data types have two different representations within Sahara:

* **Type instances** are compact representations of each constructed data type used by Sahara at runtime
* **Type definitions** are used at compile time to generate instructions that result in efficient runtime operations

## Layout

### Type instances

Type instances can be stored either on the [stack](./static-memory.md) or the [heap](./dynamic-memory.md). In either
case, their runtime representation is the same. Each type instance is simply a contiguous sequence of values stored at a
specific index within the storage location.

This representation is maximally compact; only the data that is necessary to represent the fields of the data type are
stored at runtime.

### Type definitions

Because of the compact structure of type instances, additional metadata is required at compile time so that
[instructions](./bytecode.md) can interact with data types. While these definitions could technically be erased
completely at runtime, they remain useful for [metaprogramming](./metaprogramming.md) and [debugging](./debug.md), so
they are normally not stripped from bytecode even for optimized release builds.

Each type definition contains the following:

* The type name
* The total size of each type instance
* The name, type, size, and offset of each field contained within the type

A few notes on interpretation of these definitions:

* All sizes and offsets are in terms of individual bytes
* There is no guarantee that a definitions total size is equal to the sum of its field sizes, as padding my be added by
  the VM implementation
* Fields should generally be ordered from largest to smallest size to maximize instance density


### Example

Consider the following Jackal `data` forms:

```lisp
(module colors)

(data Rgb
 [red U8]
 [green U8]
 [blue U8])

(data Color
 [name String]
 [rgb Rgb])
```

The type definition for `Rgb`:

```yaml
name: "colors::Rgb"
size: 3
fields:
  - name: red
    type: u8
    offset: 0
    size: 1
  - name: green
    type: u8
    offset: 1
    size: 1
  - name: blue
    type: u8
    offset: 2
    size: 1
``````

And for `Color`:

```yaml
name: "colors::Color"
size: 16
fields:
  - name: name
    type: string
    offset: 0
    size: 8
  - name: rgb
    type: "colors::Rgb"
    offset: 8
    size: 3
```

Leading to the type instance for a `Color` with its name interned at address `0xBAAAAAADDEADBEEF` an rgb of `0x00FFFF`:

`0xBAAAAADDEADBEEF00FFFFFF`

When accessing the type instance, a compiler should reference the type definition to determine the offset into the
instance that should be used to load or store the value of a field. From here, values can be loaded to/from the data
stack or local registers as usual. See the [data type bytecode specification](./bytecode.md#interacting-with-data-types)
for more details.
