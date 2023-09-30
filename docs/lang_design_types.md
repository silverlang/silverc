# Silver Language Design: Types

## Basic types

- `int8/int16/int32/int64` - A signed integer that is 8, 16, 32 and 64 bits long, respectively.
- `uint8/uint16/uint32/uint64` - An unsigned integer.
- `int` - A type alias for `int32`. 
- `uint` - A type alias for `uint32`.
- `bool` - Boolean type. Literal names are `True` and `False`.
- `float` - A floating number that is 64 bits long.

- `str` - String type. # TODO: clarify string type more

## Compound types

- Tuple type. A tuple can never grow or shrink in size once created. The values (or references) inside a tuple cannot be changed.
    - Example of a tuple with 3 elements: `(T, U, V)`
    - Example of a tuple with a certain length: `(T: 5)`
- `list` - A mutable array type with elements of the same type.
    -  Type syntax: `list[T]`
- `dict` - A collection of key-value pairs.
    -  Type syntax: `dict[T, U]`

## Other types

- `Self` - Used in method function signatures, and refers to the struct that owns the method.

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
    capacity: uint = 0
    size: uint

struct Mapping[T, U]:
    key: T
    value: U
```

### Generic constraints
Generics can have constraints to limit which types are acceptable.

```crystal
def subtract[T: Signed](a: T, b: T) -> T: # will accept all int (not uint) types
    return a + b

def add[T: Ordered](a: T, b: T) -> T: # will accept all int, uint, and float types
    return a + b
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
type AsciiChar = uint8
type Callback = def(TcpListener, str) -> TcpResult
```
