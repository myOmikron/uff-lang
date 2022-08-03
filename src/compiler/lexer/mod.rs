use regex::Captures;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod reg;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Token {
    Assign = 0,
    Ident,
    Integer,
    Float,
    String,
    Comment,
    Say,
    TheAnswerIs,
    EOL,
    Unknown,
}

#[derive(Debug)]
pub struct Tokenized {
    pub line: usize,
    pub start: usize,
    pub stop: usize,
    pub token: Token,
    pub value: String,
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
    ) || t == Token::Comment
    {
        if !check_if_in_bounds(
            &cap.get(0).unwrap().start(),
            &cap.get(0).unwrap().end(),
            &boundaries,
            t == Token::Comment,
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
            if t != Token::Comment {
                boundaries.push((cap.get(0).unwrap().start(), cap.get(0).unwrap().end()));
            } else {
                boundaries.push((cap.get(0).unwrap().start(), cap.get(0).unwrap().end()));
            }
        }
    }
}

pub fn lex(path: &Path) -> Vec<Tokenized> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut tokenized: Vec<Tokenized> = Vec::new();
    let mut boundaries: Vec<(usize, usize)> = Vec::new();
    let mut rem_indices_tok: Vec<usize> = Vec::new();
    let mut line: String;

    for e in reader.lines().enumerate() {
        let mut line_tokenized: Vec<Tokenized> = Vec::new();
        boundaries.clear();

        line = e.1.unwrap();

        // String
        let string_captures = reg::RE_STRING.captures_iter(&line);
        for cap in string_captures {
            append_match(
                &mut line_tokenized,
                &cap,
                Token::String,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // Comments
        rem_indices_tok.clear();
        let comment_captures = reg::RE_LINE_COMMENT.captures_iter(&line);
        for capture in comment_captures {
            for i in 0..line_tokenized.len() {
                if line_tokenized[i].line == e.0 + 1
                    && line_tokenized[i].token == Token::String
                    && line_tokenized[i].start >= capture.get(0).unwrap().start() + 1
                {
                    rem_indices_tok.push(i);
                }
            }

            rem_indices_tok.sort_by(|x, y| y.partial_cmp(x).unwrap());

            for i in &rem_indices_tok {
                line_tokenized.remove(*i);
            }

            append_match(
                &mut line_tokenized,
                &capture,
                Token::Comment,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // decimal float
        let decimal_float_captures = reg::RE_DECIMAL_FLOAT.captures_iter(&line);
        for capture in decimal_float_captures {
            append_match(
                &mut line_tokenized,
                &capture,
                Token::Float,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // integer
        let integer_captures = reg::RE_INTEGER.captures_iter(&line);
        for capture in integer_captures {
            append_match(
                &mut line_tokenized,
                &capture,
                Token::Integer,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // assign
        let assign_captures = reg::RE_ASSIGN.captures_iter(&line);
        for capture in assign_captures {
            append_match(
                &mut line_tokenized,
                &capture,
                Token::Assign,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // say
        let say_captures = reg::RE_SAY.captures_iter(&line);
        for capture in say_captures {
            append_match(
                &mut line_tokenized,
                &capture,
                Token::Say,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        // the answer is
        let the_answer_is_captures = reg::RE_THE_ANSWER_IS.captures_iter(&line);
        for capture in the_answer_is_captures {
            append_match(
                &mut line_tokenized,
                &capture,
                Token::TheAnswerIs,
                &line,
                &e.0,
                &mut boundaries,
            )
        }

        // identifier
        let ident_captures = reg::RE_IDENT.captures_iter(&line);
        for capture in ident_captures {
            append_match(
                &mut line_tokenized,
                &capture,
                Token::Ident,
                &line,
                &e.0,
                &mut boundaries,
            );
        }

        line_tokenized.sort_by(|x, y| x.start.cmp(&y.start));

        // unknown
        let mut last: usize = 0;
        let mut unknown: Vec<Tokenized> = Vec::new();
        for token in &line_tokenized {
            let value = &line[last..token.start - 1];
            if value != "" && !reg::RE_WHITESPACE.is_match(&value) {
                unknown.push(Tokenized {
                    line: e.0 + 1,
                    start: last,
                    stop: token.start,
                    token: Token::Unknown,
                    value: String::from(value.trim()),
                });
            }
            last = token.stop - 1;
        }

        // If either no other Tokens are found or there's something unknown at the end
        // execute UNKNOWN match one more time
        if last < line.len() - 1 {
            let value = &line[last..line.len()];
            if !reg::RE_WHITESPACE.is_match(&value) {
                unknown.push(Tokenized {
                    line: e.0 + 1,
                    start: last,
                    stop: line.len(),
                    token: Token::Unknown,
                    value: String::from(value.trim()),
                });
            }
        }

        line_tokenized.append(&mut unknown);
        line_tokenized.sort_by(|x, y| x.start.cmp(&y.start));

        line_tokenized.push(Tokenized {
            token: Token::EOL,
            value: String::from(""),
            line: e.0 + 1,
            start: line.len(),
            stop: line.len(),
        });

        tokenized.extend(line_tokenized);
    }

    return tokenized;
}
