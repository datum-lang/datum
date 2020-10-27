# 5. types system

Date: 2020-10-27

## Status

2020-10-27 proposed

## Context

Go examples:

```golang
type uint uint
type byte = uint8
type rune = int32
```

in Go:

```markdown
Integers
    Signed
        int
        int8
        int16
        int32
        int64
    Unsigned
        uint
        uint8
        uint16
        uint32
        uint64
        uintptr
Floats
    float32
    float64
Complex Numbers
    complex64
    complex128
Byte
Rune
String
Boolean
```

Rust:

```
bool : The boolean type.
char : A character type.
i8 : The 8-bit signed integer type.
i16 : The 16-bit signed integer type.
i32 : The 32-bit signed integer type.
i64 : The 64-bit signed integer type.
isize : The pointer-sized signed integer type.
u8 : The 8-bit unsigned integer type.
u16 : The 16-bit unsigned integer type.
u32 : The 32-bit unsigned integer type.
u64 : The 64-bit unsigned integer type.
usize : The pointer-sized unsigned integer type.
f32 : The 32-bit floating point type.
f64 : The 64-bit floating point type.
array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
slice : A dynamically-sized view into a contiguous sequence, [T].
str : String slices.
tuple : A finite heterogeneous sequence, (T, U, ..).
```

## Decision

 - `bool` can be either `true` or 'false'

## Consequences

Consequences here...
