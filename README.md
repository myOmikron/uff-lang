# uff-lang

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
`'`: Block comment
`"`: Line comment

### Datatypes
#### Strings
Strings have to begin with `<<` and end with `>>`. The use of `>>` in the string is not possible.

#### complex128
2x `float64`, imag + real

#### float64

#### int64

#### bigint

#### array[generic]
Access: `b@2` or `b@-2`

#### struct
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