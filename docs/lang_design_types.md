# Silver Language Design: Types

## Basic types

- `int` - This maps directly to the Wasm type `i32` , signed
- `int64` - This maps directly to the Wasm type `i64`, signed
- `uint` - This maps directly to the Wasm type `i32` , unsigned
- `uint64` - This maps directly to the Wasm type `i64`, unsigned
- `bool` - Boolean type. Literal names are `True` and `False`. In Wasm, booleans are just integers `0` and `1`
- `float` - This maps directly to the Wasm type `f32`
- `float64` - This maps directly to the Wasm type `f64`

## Compound types

- `Tuple` type. 
    - A tuple can never grow or shrink in size once created. The values (or references) inside a tuple cannot be changed.
        - Example of a tuple with 3 elements: `(T, U, V)`
        - Example of a tuple with a certain length: `(T: 5)`
- `Struct` type
    - A struct can be declared as a new compound type which will be put on the stack
        - Example
        ```crystal
        struct Client:
            socket: socket.Socket 
            messages: List[String]
        ```
- `String` type 
    - In Wasm, a string is not a primitive, so we must find a way to separate string literals and string types.
    - A solution to this would be to write the String type in the core library to use a memory buffer
        and put literals somewhere in the binary (maybe the data section)
        - TODO: Learn more about how the data section works so we can statically store string literals

- `Array` type 
    - Wasm has a different way to handle arrays, unlike with machine level assembler languages
    - Question: Perhaps, due to the lack of array support in Wasm, should we instead use the core library to implement a List type?
    - TODO: Write some experimental code to find out how we could possibly implement an array primitive for Wasm

## Other types

- `Self` - Used in method function signatures, and refers to the struct that owns the method.
- `Memory` - Used to invoke the [*bulk memory operations*](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format#bulk_memory_operations)

## Generics
Generics abstract over types in functions and structs when the data type is not known at compile time.
```crystal
from core import mem.MemBuf

# function generics
def create_list[T](size: uint) -> List[T]:
    return List[T](size)

# struct generics
struct List[T]:
    buffer: MemBuf = MemBuf(1024 * 1024 * 1024)
    capacity: int = 0
    size: int

struct Map[T, U]:
    key: T
    value: U
    capacity: int = 0
    size: int
```

### Generic constraints
Generics can have constraints to limit which types are acceptable.

```crystal
struct Iterator[I, T: Collection[I]]:
    subject: T
    item: I

    def map[R](self, transform: def(I) -> R) -> Map[R]:
        mapper = Map()
        mapper.apply(transform)
        return mapper
```

## Function types

Function types can be specified with the syntax being similar to the actual function signature.

```crystal
def subtract_num(subtrahend: int) -> def(int) -> int:
    return def(minuend) int(minuend - subtrahend)
```

## Type Aliases
Type aliases are declared similarly to regular variables, with the `type` prefix keyword.

```crystal
type AsciiChar = int
type Callback = def(TcpListener, str) -> TcpResult
```
