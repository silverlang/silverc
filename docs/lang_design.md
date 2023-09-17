# Silver Language Design Proposal 1
This document shows the initial proposal for the silver language syntax and features as the out-of-the-box feature-set.

## Project Structure and Imports
The default project structure can be overridden by framework plugins to allow users of said framework to easily follow the project's structure.
The default structure is as follows:
```
root
  |- src 
      |- main.ag
      |- modA
          |- modA.ag
          |- modA1.ag
          |- modA2.ag
      |- modB
          |- modB.ag
          |- modB1.ag
          |- modB2.ag
```
A module can either be a directory or a file. Unlike in rust, a directory is not required to have a main mod file `mod.ag`. 
However, one can bind symbols to the directory's module by creating a file with the same named as the directory.
So in other words, symbols within `modA/modA.ag` can be accessed by importing the directory itself
```python
import modA.Animal
```
As opposed to accessing it through the file itself
```python
import modA.modA.Animal
```
Files within a directory are not required to be declared in the main mod file, like in rust.
Accessing symbols within `modA/modA1.ag` can be imported as long as it exists.
```python
import modA.modA1.Server #No module declaration needed
```
This makes it easier for people to quickly add new files to their project and just use it without having to go to the main mod file. 
Not a big inconvenience but it is (in my opinion at least) quite tedious and annoying.

The name `src` as the root source directory is not enforced, it's just the default. 
This can be overridden via configurations (in silverc, the flag `-src=[src_dir_name]`).

Of course, for libraries, `main.ag` would not be needed. Unlike in rust, `lib.ag` would not be necessary as a simple cli flag can be enough to declare the project a library.

For libraries, importing them requires you to specify the name of the library in a `from` clause.
```python
from core import net.socket

socket = socket.Tcp4Socket("localhost", 8888)
```

This is sugar for `import core.net.socket`.

## Variables
Variables are simply named bindings to values, more specifically, values on the stack. The values should be restricted to a certain type to guarantee the correct operations can be executed.
So the variable syntax will look a lot like python variables with type hints, except the type hint is not at all a hint.
```python
name: str = "Alex"
```

There is no `null` value in silver, however, `None` can take the place of `null` and allow plugins to extend the semantics of `None`.
Setting a variable to `None` without a type will result in an error. A `None` will allocate the space needed for that type and 0-initialize it.

```python 
person: Person = None #Will compile
person = None         #Will not compile
```
This means one can still use `person` normally, however, the compiler will emit a warning or an error if it sees the variable used normally while still being set to `None`.
```python
person: Person = None
person.work() #Person receiver `person` is set to None
```

Variables with a non-None value can be set without a type annotation
```python
person = Person("alex", 25)
```

Variables can also be set to an if-else branch to allow for conditional initialization
```python
person = if self.database.has(name):
            self.database.get(name)
        else:
            self.database.set(name, default_person)
```

## Functions
Functions will look a lot like python functions with type-hints, however, the type hints are not hints.
```python 
def register(person: Person) -> RegistryResult:
    ...
```

Functions can take any number of parameters and any number of return values
```python
def get_user_session(username: str) -> DatabaseResult, AccountSession:
    ...
```
Functions can be referenced like variables to create a `function reference`, which can allow a user to pass around arbitrary functions to be used.
```python
listener = TcpListener()
listener.register_message_received_callback(message_received_callback)

def message_received_callback(listener: TcpListener, message: str) -> TcpResult:
    ...
```
However, unlike in Python, one must specify the type signature of a function to accept them.
Generally, the syntax is like this: `def(TcpListener, str) -> TcpResult`.
```python
def register_callback(callback: def(TcpListener, str) -> TcpResult):
    ...
```
Functions can also be anonymous using a similar syntax to the signature. In fact, the anonymous function 
is just the signature syntax with a body, with the input types replaced with variable names:
```python
register_callback(def(listener, message) TcpResult(message.parseInt()))
```
The body can either be deliniated by a `:` plus a newline as you see below, or no colon but an expression on the same line, as you see above.
```python
register_callback(def(listener, message):
    msg, result: ServerMessage, JsonResult = json.parse(message)
    if result.is_err():
        return TcpResult.error(result, "Failed to parse json message")
    process_message(msg)
    return TcpResult.ok()
)
```

## Structs
Structs are essentially the same as `class` in python, however, I feel that the term `struct` better describes what is being declared.
A struct can be declared with some fields in the body, which can either be left empty, meaning that are required to be filled upon construction,
or they can be given a value
```crystal
struct Person:
    name: str 
    age: uint

person = Person("Alex", 25) #Compiles
person = Person() #Does not compile, missing arguments to constructor
```
A struct can have functions in them which turns the struct into a sub-module which contains the functions.
These are called *associated functions*.
```crystal
struct Person:
    name: str 
    age: uint 

    def from_json(data: JsonString) -> Self:
        return json.parse(data)

person = Person.from_json(data)
```
A struct can also have functions bound to an instance of it, these are methods.
To declare a method, simply define a function within a struct's body with its first parameter as `self`
```crystal
struct Person:
    name: str 
    age: uint 

    def from_json(data: JsonString) -> Self:
        return json.decode(data)

    def to_json(self) -> JsonString:
        return json.encode(self)

data = server.receive_json()
person = Person.from_json(data)

data = person.to_json()
server.send_json(data)
```

Structs can be *composed* of other structs, which allows one struct to receive the structure and methods of another.
```crystal
# A Composable struct
struct JsonComponent:
    tag: JsonTag

    def encode(self, JsonEncoder) -> JsonResult:
        ...

    def decode(self, JsonDecoder) -> JsonResult:
        ...

#Receives all fields and methods of JsonComponent, aka, a Composed Struct
struct JsonObject(JsonComponent):
    children: List[JsonComponent]

#Receives all fields and methods of JsonComponent
struct JsonValue(JsonComponent):
    name: str 
    value: JsonValue
```

Structs are considered "composable". In the compiler, a Composable trait can be provided to plugins for creating custom Composables.
A composable struct with unimplemented functions or methods must be implemented by the Composed struct.

```crystal
# A Composable struct
struct JsonComponent:
    tag: JsonTag

    def encode(self, JsonEncoder) -> JsonResult

    def decode(self, JsonDecoder) -> JsonResult

#Receives all fields and methods of JsonComponent, aka, a Composed Struct
struct JsonObject(JsonComponent):
    children: List[JsonComponent]

    def encode(self, JsonEncoder) -> JsonResult:
        ...

    def decode(self, JsonDecoder) -> JsonResult:
        ...

#Receives all fields and methods of JsonComponent
struct JsonValue(JsonComponent):
    name: str 
    value: JsonValue

    def encode(self, JsonEncoder) -> JsonResult:
        ...

    def decode(self, JsonDecoder) -> JsonResult:
        ...
```
With this system, you can treat composables as like interfaces or traits. One can pass an instance of struct 
composed of another struct into a function which takes the parent.
```python
def encode_json(root: JsonComponent) -> JsonString:
    ...

root: JsonValue = database.get(username).as_json()
json_str = encode_json(root)
```

## Enums
Enumerable types are good for assigning a unique type to a set of related values.
This is used for creating tags which can be matched on when processing data.
```crystal
enum JsonTag:
    JsonObject = 0,
    JsonNumber,
    JsonString,
    JsonFloat
```
Enums are not considered "composables".
```crystal
enum JsonTag:
    JsonObject = 0,
    JsonNumber,
    JsonString,
    JsonFloat

#Error: enums cannot be composed
enum MyJsonTag(JsonTag):
    ...
```
Enums can be used like any other type:
```crystal
struct JsonComponent:
    tag: JsonTag

    def get_tag(self) -> JsonTag:
        return self.tag

    def has_tag(self, tag: JsonTag) -> bool:
        return self.tag == tag
```
Enums by default allow for comparing values, as you see above. One can also do ordered comparisons.
```python
def check_tag(tag: MyTag) -> bool:
    return tag > 0
```

Enums can only be of primitive types, such as strings, chars, floats, etc
```crystal
enum ErrorMessage:
    message_too_long = "Message cannot exceed 200 characters"
    message_invalid = "Message could not be processed into json"
    user_does_not_exist = "User %s does not exist"
```

## Generics
Generics are a great way to abstract over different types of data when the data type is not known at compile time.
It can be declared in functions and structs
```crystal
from core import mem.MemBuf

def create_list[T](size: uint) -> List[T]:
    return List[T](size)

struct List[T]:
    buffer: MemBuf = MemBuf(1024 * 1024 * 1024)
    capacity: uint = 0
    size: uint

    def push(self, data: T):
        ...

    def get(self, index: uint) -> T:
        ...
```

## Type Aliases
Type aliases are useful for renaming a type within a module for convenience, such as with function types.
```python
type Callback = def(TcpListener, str) -> TcpResult

def register_callback(callback: Callback):
    ...
```
