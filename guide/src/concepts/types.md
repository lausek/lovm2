# Types

`lovm2` embraces dynamic typing. The most basic ones are `Bool`, `Int`, `Float`, `String`.

`Nil` is the default return type of functions that do not have return values. You can also use it to mark the absence of a value.

`Ref` can wrap any other value and implement a shared mutable location as such.

## Complex Types

`List` and `Dict` are a bit more complicated, because they need to store other values. As such, they support the  `contains`, `len`, `get`, `set` and `delete` methods.

These types also utilize `Ref` heavily. If you use the standard `lovm2` functionality for generating programs, you will always implicitly work with a `Ref` to the corresponding data. The virtual machine also ensures that every value being stored inside these types is itself wrapped up in a reference. This is required for the implementation of slices. The `Box` instruction realizes this functionality.

Another special value kind is `Any`. This type is used for allowing external shared object modules to pass their custom Rust structures into the VM.

## Conversion

The `Conv` instruction is able to convert data according to the following rules:

|from / to| Nil | Bool | Int | Float | String | List | Dict | Ref |
|:-----:|:---:|:----:|:---:|:-----:|:------:|:----:|:----:|:---:|
| Nil   |  ✓  |      |     |       |   ✓    |      |      |     | 
| Bool  |     |  ✓   |  ✓  |       |   ✓    |      |      |     |
| Int   |     |  ✓   |  ✓  |   ✓   |   ✓    |      |      |     |
| Float |     |      |  ✓  |   ✓   |   ✓    |      |      |     |
| String|     |      |  ✓¹  |   ✓¹   |   ✓    |      |      |     |
| List  |     |      |     |       |   ✓    |  ✓   |      |     |
| Dict  |     |      |     |       |   ✓    |      |  ✓   |     |
| Ref   |     |      |     |       |   ✓    |      |      |  ✓  |

*¹ implies parsing overhead*
