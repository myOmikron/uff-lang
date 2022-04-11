# uff-lang

To add new proposals, open a new issue, describe what and why we should change. 
Add an example as well as explain the use case to archive.

## Grammar

```ebnf
identifier = (unicode_symobls | unicode_letter) { unicode_symobls | unicode_letter | unicode_digit } .
imaginary = ( decimal_float | integer ) "i" .
decimal_float = integer "." digits [ "e" integer ] .
integer = [ "-" ] digits .
digits = digit { ["_"] digit } .
digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 .
unicode_symobls = /* Unicode 13.0.0 Character Classes: Sm, Sc, Sk, So */ .
unicode_digit = /* Unicode 13.0.0 Characer Classes: Nd */ | "_" .
unicode_letter = /*Unicode 13.0.0 Characer Classes: Lu, Ll, Lt, Lm, Lo */.
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

### Required keywords
These should be valid keywords, but I don't have an idea, what to use them for:

- `xd`
- `uff`
