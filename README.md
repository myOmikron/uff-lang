# uff-lang

To add new proposals, open a new issue, describe what and why we should change. 
Add an example as well as explain the use case to archive.

## How to use

- Don't use it.

But if you really insist:

`uff.uff`
```uff
should should be <<be>>
be should be <<should>>
say be should
the answer is 0
```

```bash
~> uff build uff.uff
~> bin/uff
should be
```

## Grammar

```ebnf
exit_statement = "the" whitespace "answer" whitespace "is" whitespace ( integer | identifier ) .
assign_statement = identifier whitespace "should be" whitespace expression .
expression = ( integer | identifier | string | decimal_float ) .
string = "<<" /* any character */ ">>" .
identifier = (unicode_symobls | unicode_letter) { unicode_symobls | unicode_letter | unicode_digit } .
imaginary = ( decimal_float | integer ) "i" .
decimal_float = integer "." digits [ "e" integer ] .
integer = [ "-" ] digits .
digits = digit { ["_"] digit } .
digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 .
whitespace = whitespace_character { whitespace_character } .
whitespace_character = /* Unicode 13.0.0 Character Class: White_Space */ .
unicode_symobls = /* Unicode 13.0.0 Character Classes: Sm, Sc, Sk, So */ .
unicode_digit = /* Unicode 13.0.0 Characer Class: Nd */ | "_" .
unicode_letter = /* Unicode 13.0.0 Characer Classes: Lu, Ll, Lt, Lm, Lo */.
```

### Assign
- `should be`

### Comments [Proposal]
- `'`: Block comment
- `"`: Line comment

### Exiting
To exit the program just use the keyword: `the answer is <int64>`. 
If no explicit exit code was given, `42` is used.

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

### Exceptions
Exceptions are named heart attacks. 

### Pointer

### Generics
`tbd` as keyword

### Required keywords
These should be valid keywords, but I don't have an idea, what to use them for:

- `xd`
- `uff`
- `yeet`
