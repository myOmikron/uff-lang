use regex::Captures;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod reg;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Token {
    EOF = 0,
    EOL,
    ASSIGN,
    IDENT,
    INTEGER,
    FLOAT,
    STRING,
    COMMENT,
    SAY,
}

#[derive(Debug)]
pub struct Tokenized {
    line: usize,
    start: usize,
    stop: usize,
    token: Token,
    value: String,
}

impl Display for Tokenized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}:{} - {:?} {}",
            self.line, self.start, self.stop, self.token, self.value
        )
    }
}

fn check_start_end(line: &str, first: &usize, last: &usize) -> bool {
    if *first > 0 {
        if !reg::RE_DELIM_FRONT.is_match(&line[..*first]) {
            return false;
        }
    }

    if *last < line.len() - 1 {
        if !reg::RE_DELIM_BACK.is_match(&line[*last..]) {
            return false;
        }
    }

    return true;
}

fn check_if_in_bounds(
    first: &usize,
    last: &usize,
    boundaries: &Vec<(usize, usize)>,
    only_inside: bool,
) -> bool {
    for boundary in boundaries {
        if (*first > boundary.0 && *first < boundary.1)
            || (*last > boundary.0 && *last < boundary.1)
            || (!only_inside && *first <= boundary.0 && *last >= boundary.1)
        {
            return true;
        }
    }

    return false;
}

fn split_range(first: &usize, last: &usize, line: &str) -> String {
    return line[*first..*last].to_string();
}

fn append_match(
    vec: &mut Vec<Tokenized>,
    cap: &Captures,
    t: Token,
    line: &String,
    e: &usize,
    boundaries: &mut Vec<(usize, usize)>,
) {
    if check_start_end(
        line,
        &cap.get(0).unwrap().start(),
        &cap.get(0).unwrap().end(),
    ) {
        if !check_if_in_bounds(
            &cap.get(0).unwrap().start(),
            &cap.get(0).unwrap().end(),
            &boundaries,
            if t == Token::COMMENT { true } else { false },
        ) {
            vec.push(Tokenized {
                token: t,
                value: split_range(
                    &cap.get(1).unwrap().start(),
                    &cap.get(1).unwrap().end(),
                    line,
                ),
                line: *e + 1,
                start: cap.get(0).unwrap().start() + 1,
                stop: cap.get(0).unwrap().end() + 1,
            });
            if t != Token::COMMENT {
                boundaries.push((cap.get(0).unwrap().start(), cap.get(0).unwrap().end()));
            } else {
                boundaries.push((cap.get(0).unwrap().start(), cap.get(0).unwrap().end()));
            }
        }
    }
}

pub fn lex(path: &str) -> Vec<Tokenized> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut tokenized: Vec<Tokenized> = Vec::new();
    let mut boundaries: Vec<(usize, usize)> = Vec::new();
    let mut rem_indices_tok: Vec<usize> = Vec::new();
    let mut line: String;

    for e in reader.lines().enumerate() {
        boundaries.clear();

        line = e.1.unwrap();

        // String
        let string_captures = reg::RE_STRING.captures_iter(&line);
        for cap in string_captures {
            append_match(
                &mut tokenized,
                &cap,
                Token::STRING,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // Comments
        rem_indices_tok.clear();
        let comment_captures = reg::RE_LINE_COMMENT.captures_iter(&line);
        for capture in comment_captures {
            for i in 0..tokenized.len() {
                if tokenized[i].line == e.0 + 1
                    && tokenized[i].token == Token::STRING
                    && tokenized[i].start >= capture.get(0).unwrap().start() + 1
                {
                    rem_indices_tok.push(i);
                }
            }

            rem_indices_tok.sort_by(|x, y| y.partial_cmp(x).unwrap());

            for i in &rem_indices_tok {
                tokenized.remove(*i);
            }

            append_match(
                &mut tokenized,
                &capture,
                Token::COMMENT,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // decimal float
        let decimal_float_captures = reg::RE_DECIMAL_FLOAT.captures_iter(&line);
        for capture in decimal_float_captures {
            append_match(
                &mut tokenized,
                &capture,
                Token::FLOAT,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // integer
        let integer_captures = reg::RE_INTEGER.captures_iter(&line);
        for capture in integer_captures {
            append_match(
                &mut tokenized,
                &capture,
                Token::INTEGER,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // assign
        let assign_captures = reg::RE_ASSIGN.captures_iter(&line);
        for capture in assign_captures {
            append_match(
                &mut tokenized,
                &capture,
                Token::ASSIGN,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // say
        let say_captures = reg::RE_SAY.captures_iter(&line);
        for capture in say_captures {
            append_match(
                &mut tokenized,
                &capture,
                Token::SAY,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // identifier
        let ident_captures = reg::RE_IDENT.captures_iter(&line);
        for capture in ident_captures {
            append_match(
                &mut tokenized,
                &capture,
                Token::IDENT,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // push EOL
        tokenized.push(Tokenized {
            token: Token::EOL,
            line: e.0 + 1,
            start: line.len(),
            stop: line.len() + 1,
            value: "".parse().unwrap(),
        })
    }

    // swap last EOL with EOF
    if tokenized.len() > 0 {
        tokenized.push(Tokenized {
            token: Token::EOF,
            line: tokenized[tokenized.len() - 1].line,
            start: tokenized[tokenized.len() - 1].start,
            stop: tokenized[tokenized.len() - 1].stop,
            value: "".parse().unwrap(),
        });
        tokenized.swap_remove(tokenized.len() - 2);
    }

    return tokenized;
}
