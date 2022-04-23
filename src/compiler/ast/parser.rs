use crate::compiler::ast::definitions::Expression;
use crate::compiler::lexer::{Token, Tokenized};

pub fn parse(vec: &Vec<Tokenized>) -> Result<Expression, String> {
    let first_opt = vec.get(0);

    if first_opt.is_none() {
        return Ok(Expression::None {});
    }

    let first = first_opt.unwrap();

    match first.token {
        Token::COMMENT => return Ok(Expression::None {}),
        Token::EOL => return Ok(Expression::None {}),
        Token::IDENT => {
            return Ok(Expression::None {});
            unimplemented!()
        }
        Token::SAY => {
            return Ok(Expression::None {});
            unimplemented!()
        }
        Token::THE_ANSWER_IS => {
            let next_opt = vec.get(1);
            if next_opt.is_none() {
                return Err(String::from("No answer given!"));
            }

            let next = next_opt.unwrap();
            return if next.token == Token::INTEGER {
                if vec.len() > 2 {
                    return Err(String::from("There can only be one answer!"));
                }

                let answer = next.value.parse::<i64>().unwrap();
                Ok(Expression::ExitCode { code: answer })
            } else if next.token == Token::IDENT {
                Ok(Expression::ExitVar {
                    variable: String::from(&next.value),
                })
            } else {
                Err(String::from("The answer must be an integer!"))
            };
        }
        Token::UNKNOWN => {
            return Err(format!(
                "Unknown token: line: {} column: {}: {}",
                first.line, first.start, first.value
            ))
        }
        _ => return Err(format!("{:?} is not allowed as first token!", first.token)),
    }
}
