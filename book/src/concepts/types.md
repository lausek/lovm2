# Types

## Simple Types

`Bool`, `Int`, `Float`

`Nil` is the default return type of functions that do not have return values.

`String`

`Ref` can wrap any other value and implement a shared mutable place as such.

## Complex Types

`List` and `Dict` are a bit more complicated, because they need to store other values. As such, they support the `len`, `get` and `set` methods.

These types also utilize `Ref` heavily. If you use the standard `lovm2` functionality for generating programs, you will always implicitly work with a `Ref` to the corresponding data. The virtual machine will also ensure that every value being stored inside these types is itself wrapped up in a reference. This is required for the implementation of slices. The `Box` instruction implements this functionality.

## Conversion

The `Conv` instruction is able to convert data according to the following rules:

|from / to| Nil | Bool | Int | Float | String | List | Dict | Ref |
|:-----:|:---:|:----:|:---:|:-----:|:------:|:----:|:----:|:---:|
| Nil   |  ✓  |      |     |       |   ✓    |      |      |     | 
| Bool  |     |  ✓   |  ✓  |       |   ✓    |      |      |     |
| Int   |     |  ✓   |  ✓  |   ✓   |   ✓    |      |      |     |
| Float |     |      |  ✓  |   ✓   |   ✓    |      |      |     |
| String|     |      |  ~  |   ~   |   ✓    |      |      |     |
| List  |     |      |     |       |   ✓    |  ✓   |      |     |
| Dict  |     |      |     |       |   ✓    |      |  ✓   |     |
| Ref   |     |      |     |       |   ✓    |      |      |  ✓  |
