use crate::compiler::ast::definitions::{Expression, Statement};
use crate::compiler::lexer::{Token, Tokenized};

fn parse_int(value: &str) -> i64 {
    value.replace("_", "").parse::<i64>().unwrap()
}

fn parse_float(value: &str) -> f64 {
    value.replace("_", "").parse::<f64>().unwrap()
}

pub fn parse(vec: &Vec<Tokenized>) -> Result<Statement, String> {
    let first_opt = vec.get(0);

    if first_opt.is_none() {
        return Ok(Statement::None {});
    }

    let first = first_opt.unwrap();

    match first.token {
        Token::Comment => return Ok(Statement::None {}),
        Token::EOL => return Ok(Statement::None {}),
        Token::Ident => {
            if vec.len() != 3 {
                return Err(String::from("Syntax: IDENT should be VALUE"));
            }

            let next_opt = vec.get(1);
            if next_opt.is_none() {
                return Err(String::from("Don't know what to do with this!"));
            }

            let next = next_opt.unwrap();
            if next.token != Token::Assign {
                return Err(String::from("Syntax: IDENT should be VALUE"));
            }

            let value_opt = vec.get(2);
            if value_opt.is_none() {
                return Err(String::from("Syntax: IDENT should be VALUE"));
            }
            let value = value_opt.unwrap();
            match value.token {
                Token::Integer => Ok(Statement::AssignInteger {
                    ident: String::from(&first.value),
                    value: parse_int(&value.value),
                }),
                Token::Float => Ok(Statement::AssignFloat {
                    ident: String::from(&first.value),
                    value: parse_float(&value.value),
                }),
                Token::String => Ok(Statement::AssignString {
                    ident: String::from(&first.value),
                    value: String::from(&value.value),
                }),
                _ => Err(String::from("Value must be one of INTEGER, FLOAT, STRING")),
            }
        }
        Token::Say => {
            let mut expressions = vec![];

            let mut next = 1;
            let mut c = true;
            while c {
                let value_opt = vec.get(next);
                match value_opt {
                    None => {
                        c = false;
                    }
                    Some(tokenized) => match tokenized.token {
                        Token::Ident => {
                            expressions.push(Expression::Ident(tokenized.value.clone()))
                        }
                        Token::Integer => {
                            expressions.push(Expression::Integer(parse_int(&tokenized.value)));
                        }
                        Token::Float => {
                            expressions.push(Expression::Float(parse_float(&tokenized.value)));
                        }
                        Token::String => {
                            expressions.push(Expression::String(tokenized.value.clone()));
                        }
                        _ => {
                            return Err(format!(
                                "{:?} not allowed here: {}:{}",
                                tokenized.token, tokenized.line, tokenized.start
                            ))
                        }
                    },
                }
                next += 1;
            }
            return Ok(Statement::Say { expressions });
        }
        Token::TheAnswerIs => {
            let next_opt = vec.get(1);
            if next_opt.is_none() {
                return Err(String::from("No answer given!"));
            }

            let next = next_opt.unwrap();
            return if next.token == Token::Integer {
                // THE_ANSWER_IS ( INTEGER / IDENT )
                if vec.len() > 2 {
                    return Err(String::from("There can only be one answer!"));
                }

                let answer = parse_int(&next.value);
                Ok(Statement::ExitCode { code: answer })
            } else if next.token == Token::Ident {
                Ok(Statement::ExitVar {
                    variable: String::from(&next.value),
                })
            } else {
                Err(String::from("The answer must be an integer!"))
            };
        }
        Token::Unknown => {
            return Err(format!(
                "Unknown token: line: {} column: {}: {}",
                first.line, first.start, first.value
            ))
        }
        _ => return Err(format!("{:?} is not allowed as first token!", first.token)),
    }
}
