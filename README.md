# uff-lang

To add new proposals, open a new issue, describe what and why we should change. 
Add an example as well as explain the use case to archive.

## Grammar

```ebnf
imaginary = ( decimal_float | integer ) "i" .
decimal_float = integer "." digits [ "e" integer ] .
integer = [ "-" ] digits .
digits = digit { ["_"] digit } .
digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 .
```

### Assign
- `should be`

### Comments [Proposal]
- `'`: Block comment
- `"`: Line comment

### Datatypes
#### Strings
Strings have to begin with `<<` and end with `>>`. The use of `>>` in the string is not possible.

#### complex128
2x `float64`, split up in imaginary and real part.

#### float64

#### int64

#### bigint

#### array[generic]
- Access: `b@2` or `b@-2` -- Not so sure about this syntax

#### struct
This may be a valid struct in `uff`. This is also subject to be discussed.
```uff
bla struct {
    " This is a line comment
    pub {
       message string
       success bool
       data object
    }
}
```

### Pointer

### Generics
`tbd` as keyword