package main

import (
	"bufio"
	"bytes"
	"fmt"
	"github.com/hellflame/argparse"
	"github.com/myOmikron/tm/lexer"
	"io/ioutil"
	"os"
)

func main() {
	parser := argparse.NewParser("tm", "", &argparse.ParserConfig{})

	infile := parser.String("", "infile", &argparse.Option{
		Positional: true,
		Required:   true,
		Help:       "Specify an input file",
	})

	lexerOnly := parser.Flag("", "lexer-only", &argparse.Option{
		Help: "Only run and output the lexer",
	})

	if err := parser.Parse(nil); err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	if *lexerOnly {
		r, _ := ioutil.ReadFile(*infile)
		l := lexer.New(bufio.NewScanner(bytes.NewReader(r)))
		l.Run()
	}
}
