# Values

The [data stack](./execution-context.md#data-stack), [local storage](./execution-context.md#local-storage), [constant
pool](./execution-context.md#constant-pool) and [heap](./execution-context.md#heap) share a uniform representation of
supported primitive types called `Value`. `Value` provides a compact representation of data required by all Sahara
instructions that strikes a balance between expressivity and space utilization.

## Primitive types

The following primitive types are supported by `Value`:

| Type | Description             |
|------|-------------------------|
| bool | Boolean (true/false)    |
| char | 8 bit ASCII character   |
| u8   | 8 bit unsigned integer  |
| u16  | 16 bit unsigned integer |
| u32  | 32 bit unsigned integer |
| u64  | 64 bit unsigned integer |
| i8   | 8 bit signed integer    |
| i16  | 16 bit signed integer   |
| i32  | 32 bit signed integer   |
| i64  | 64 bit signed integer   |
| f32  | 32 bit floating point   |
| f64  | 64 bit floating point   |

## Compound types

| Type   | Description                                                                         |
|--------|-------------------------------------------------------------------------------------|
| string | [A sequence of characters](https://en.wikipedia.org/wiki/String_(computer_science)) |
| data   | An arbitrary [data type](./data-types.md)                                           |