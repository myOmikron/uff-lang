package lexer

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"os"
	"regexp"
)

type Token int

const (
	EOF Token = iota
	EOL
	WS
	ASSIGN
	IDENTIFIER
	INTEGER
	FLOAT
	STRING
	COMMENT
	UNKNOWN
	SAY
)

var tokenStrings = []string{
	"EOF",
	"EOL",
	"WS",
	"ASSIGN",
	"IDENTIFIER",
	"INTEGER",
	"FLOAT",
	"STRING",
	"COMMENT",
	"UNKNOWN",
	"SAY",
}

func (t Token) String() string {
	return tokenStrings[t]
}

type Lexer struct {
	position *Position
	scanner  *bufio.Scanner
	tokens   []*Tokenized
}

type Position struct {
	Line   int
	Column int
}

type Tokenized struct {
	Position Position
	Token    Token
	Value    string
}

func (l *Lexer) handleReaderError(err error) *Tokenized {
	if err != nil {
		if errors.Is(err, io.EOF) {
			return &Tokenized{
				Position: *l.position,
				Token:    EOF,
			}
		} else {
			fmt.Printf("%d:%d: %s\n", l.position.Line, l.position.Column, err.Error())
			os.Exit(1)
		}
	} else {
		return nil
	}
	return nil
}

var (
	reLineComment  = regexp.MustCompile(`(".*)`)
	reString       = regexp.MustCompile(`(<<.*?>>)`)
	reAssign       = regexp.MustCompile(`(should\s+be)`)
	reSay          = regexp.MustCompile(`(say)`)
	reInteger      = regexp.MustCompile(`(-?[0-9](?:[0-9]|_?[0-9])*)`)
	reDecimalFloat = regexp.MustCompile(`(-?[0-9](?:[0-9]|_?[0-9])*\.[0-9](?:[0-9]|_?[0-9])*(?:e-?[0-9](?:[0-9]|_?[0-9])*)?)`)
	reIdentifier   = regexp.MustCompile(`([_\p{Sm}\p{Sc}\p{Sk}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}][_\p{Sm}\p{Sc}\p{Sk}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nd}]*)`)
	reWhitespace   = regexp.MustCompile(`\s+`)
	reDelimiter    = regexp.MustCompile(`^|\s|"|$`)
)

func checkStartEnd(line string, first int, last int) (b bool) {
	b = true

	if first-1 >= 0 {
		if !reDelimiter.MatchString(string(line[first-1])) {
			b = false
		}
	}

	if last+1 <= len(line)-1 {
		if !reDelimiter.MatchString(string(line[last+1])) {
			b = false
		}
	}

	return
}

func checkIfInBounds(start int, stop int, bounds *[][]int) bool {
	for _, boundaries := range *bounds {
		if (start >= boundaries[0] && start <= boundaries[1]) || (stop >= boundaries[0] && stop <= boundaries[1]) {
			return true
		}
	}

	return false
}

func (l *Lexer) lexLine(line string) {
	delimiters := make([][]int, 0)

	// strings
	if stringIndices := reString.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if checkStartEnd(line, stringIndex[0], stringIndex[1]) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: STRING,
					Value: line[stringIndex[0]+2 : stringIndex[1]-1],
				})
			}
			delimiters = append(delimiters, []int{stringIndex[0], stringIndex[1]})
		}
	}

	// lineComment
	if stringIndices := reLineComment.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[0], &delimiters) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: COMMENT,
					Value: line[stringIndex[0]+1:],
				})
			}
			// Remove all tokens from token list which come after the line comment
			// TODO: Think of a more efficient way
			cleaned := make([]*Tokenized, 0)
			for _, token := range l.tokens {
				if token.Token == STRING {
					if token.Position.Line != l.position.Line || token.Position.Column < stringIndex[0] {
						cleaned = append(cleaned, token)
					}
				} else {
					cleaned = append(cleaned, token)
				}
			}
			l.tokens = cleaned

			delimiters = append(delimiters, []int{stringIndex[0], len(line) - 1})
		}
	}

	// decimal float
	if stringIndices := reDecimalFloat.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[1], &delimiters) && checkStartEnd(line, stringIndex[0], stringIndex[1]) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: FLOAT,
					Value: line[stringIndex[0] : stringIndex[1]+1],
				})
				delimiters = append(delimiters, []int{stringIndex[0], stringIndex[1]})
			}
		}
	}

	// integer
	if stringIndices := reInteger.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[1], &delimiters) && checkStartEnd(line, stringIndex[0], stringIndex[1]) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: INTEGER,
					Value: line[stringIndex[0] : stringIndex[1]+1],
				})
				delimiters = append(delimiters, []int{stringIndex[0], stringIndex[1]})
			}
		}
	}

	// assign
	if stringIndices := reAssign.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[1], &delimiters) && checkStartEnd(line, stringIndex[0], stringIndex[1]) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: ASSIGN,
				})
				delimiters = append(delimiters, []int{stringIndex[0], stringIndex[1]})
			}
		}
	}

	// say
	if stringIndices := reSay.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[1], &delimiters) && checkStartEnd(line, stringIndex[0], stringIndex[1]) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: SAY,
				})
				delimiters = append(delimiters, []int{stringIndex[0], stringIndex[1]})
			}
		}
	}

	// identifier
	if stringIndices := reIdentifier.FindAllStringSubmatchIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[1], &delimiters) && checkStartEnd(line, stringIndex[0], stringIndex[1]) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: IDENTIFIER,
					Value: line[stringIndex[0] : stringIndex[1]+1],
				})
				delimiters = append(delimiters, []int{stringIndex[0], stringIndex[1]})
			}
		}
	}

	// whitespaces
	if stringIndices := reWhitespace.FindAllStringIndex(line, -1); stringIndices != nil {
		for _, stringIndex := range stringIndices {
			stringIndex[1]--
			if !checkIfInBounds(stringIndex[0], stringIndex[1], &delimiters) {
				l.tokens = append(l.tokens, &Tokenized{
					Position: Position{
						Line:   l.position.Line,
						Column: stringIndex[0] + 1,
					},
					Token: WS,
				})
			}
		}
	}
}

func (l *Lexer) Run() []*Tokenized {
	var line string
	if l.scanner.Scan() {
		line = l.scanner.Text()
		l.lexLine(line)
	}

	for l.scanner.Scan() {
		l.tokens = append(l.tokens, &Tokenized{
			Position: Position{
				Line:   l.position.Line,
				Column: len(line) + 1,
			},
			Token: EOL,
		})

		l.position.Line++
		l.position.Column = 1

		line = l.scanner.Text()
		l.lexLine(line)
	}

	if err := l.scanner.Err(); err != nil {
		panic(err)
	} else {
		l.tokens = append(l.tokens, &Tokenized{
			Position: Position{
				Line:   l.position.Line,
				Column: len(line) + 1,
			},
			Token: EOF,
		})
	}

	for _, token := range l.tokens {
		fmt.Printf("%d:%d %s %s\n", token.Position.Line, token.Position.Column, token.Token.String(), token.Value)
	}
	return l.tokens
}

func New(in *bufio.Scanner) *Lexer {
	return &Lexer{
		position: &Position{
			Line:   1,
			Column: 1,
		},
		scanner: in,
		tokens:  make([]*Tokenized, 0),
	}
}
